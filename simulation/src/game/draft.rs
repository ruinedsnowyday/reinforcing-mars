use crate::game::game::Game;
use crate::player::PlayerId;

/// Draft type - determines the behavior of the draft
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftType {
    /// Initial drafting phase (project cards and optionally preludes)
    Initial,
    /// Standard drafting for subsequent generations
    Standard,
    /// Prelude drafting (only in initial draft if prelude draft variant enabled)
    Prelude,
}

/// Pass direction for drafting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassDirection {
    /// Pass to the next player (after, clockwise)
    After,
    /// Pass to the previous player (before, counter-clockwise)
    Before,
}

/// Draft implementation
impl Game {
    /// Start a draft iteration
    /// For initial draft round 1, draws cards for all players
    /// For subsequent rounds, passes cards between players
    pub fn start_draft(&mut self, draft_type: DraftType) -> Result<(), String> {
        if self.players.is_empty() {
            return Err("Cannot start draft with no players".to_string());
        }

        if self.draft_round == 1 {
            // First round: draw cards for all players
            let player_ids: Vec<PlayerId> = self.players.iter().map(|p| p.id.clone()).collect();
            let mut cards_per_player: Vec<Vec<String>> = Vec::new();
            
            for player_id in &player_ids {
                let cards = self.draw_draft_cards(draft_type, player_id.clone())?;
                cards_per_player.push(cards);
            }

            // Assign cards to players
            for (player, cards) in self.players.iter_mut().zip(cards_per_player) {
                player.draft_hand = cards;
                player.needs_to_draft = true;
            }
        } else {
            // Subsequent rounds: pass cards between players
            self.pass_draft_cards(draft_type)?;
        }

        Ok(())
    }

    /// Draw cards for a player based on draft type
    fn draw_draft_cards(&mut self, draft_type: DraftType, _player_id: PlayerId) -> Result<Vec<String>, String> {
        match draft_type {
            DraftType::Initial => {
                // Initial draft: 5 project cards per player
                // For now, return placeholder card IDs
                // TODO: Integrate with actual card deck when implemented
                Ok((0..5).map(|i| format!("project_card_{i}")).collect())
            }
            DraftType::Standard => {
                // Standard draft: 4 project cards per player
                Ok((0..4).map(|i| format!("project_card_{i}")).collect())
            }
            DraftType::Prelude => {
                // Prelude draft: typically 4 prelude cards per player
                // TODO: Get from dealt prelude cards
                Ok((0..4).map(|i| format!("prelude_card_{i}")).collect())
            }
        }
    }

    /// Pass draft cards between players based on pass direction
    fn pass_draft_cards(&mut self, draft_type: DraftType) -> Result<(), String> {
        let direction = self.get_pass_direction(draft_type);
        let mut hands: Vec<Vec<String>> = self.players.iter().map(|p| p.draft_hand.clone()).collect();

        match direction {
            PassDirection::After => {
                // Pass to next player (clockwise): rotate right
                if let Some(last) = hands.pop() {
                    hands.insert(0, last);
                }
            }
            PassDirection::Before => {
                // Pass to previous player (counter-clockwise): rotate left
                if !hands.is_empty() {
                    let first = hands.remove(0);
                    hands.push(first);
                }
            }
        }

        // Assign rotated hands back to players
        for (player, hand) in self.players.iter_mut().zip(hands) {
            player.draft_hand = hand;
            player.needs_to_draft = true;
        }

        Ok(())
    }

    /// Get the pass direction for the current draft
    fn get_pass_direction(&self, draft_type: DraftType) -> PassDirection {
        match draft_type {
            DraftType::Initial => {
                // Initial draft iteration 2 passes before, others pass after
                if self.initial_draft_iteration == 2 {
                    PassDirection::Before
                } else {
                    PassDirection::After
                }
            }
            DraftType::Standard => {
                // Standard draft alternates by generation (even = after, odd = before)
                if self.generation % 2 == 0 {
                    PassDirection::After
                } else {
                    PassDirection::Before
                }
            }
            DraftType::Prelude => {
                // Prelude draft always passes after
                PassDirection::After
            }
        }
    }

    /// Get the number of cards a player should keep in this draft round
    pub fn cards_to_keep(&self, draft_type: DraftType, _player_id: &PlayerId) -> u32 {
        match draft_type {
            DraftType::Initial => 1,
            DraftType::Standard => {
                // Standard draft: always keep 1 card per round
                1
            }
            DraftType::Prelude => 1,
        }
    }

    /// Process a player's draft selection
    /// Returns true if all players have drafted and we can proceed
    pub fn process_draft_selection(
        &mut self,
        player_id: &PlayerId,
        selected_cards: Vec<String>,
        draft_type: DraftType,
    ) -> Result<bool, String> {
        // Get cards_to_keep before borrowing
        let cards_to_keep = self.cards_to_keep(draft_type, player_id);
        
        if selected_cards.len() != cards_to_keep as usize {
            return Err(format!(
                "Expected {} cards, got {}",
                cards_to_keep,
                selected_cards.len()
            ));
        }

        let player = self
            .get_player_mut(player_id)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        // Validate that all selected cards are in draft hand
        for card_id in &selected_cards {
            if !player.draft_hand.contains(card_id) {
                return Err(format!("Card {card_id} not in draft hand"));
            }
        }

        // Move selected cards to drafted_cards
        for card_id in &selected_cards {
            player.drafted_cards.push(card_id.clone());
            player.draft_hand.retain(|c| c != card_id);
        }

        player.needs_to_draft = false;

        // Check if all players have drafted
        let all_drafted = !self.players.iter().any(|p| p.needs_to_draft);

        if all_drafted {
            // Check if there are more cards to pass
            let has_remaining_cards = self.players.iter().any(|p| p.draft_hand.len() > 1);

            if has_remaining_cards {
                // More rounds to go
                self.draft_round += 1;
                self.start_draft(draft_type)?;
                Ok(false) // Not done yet
            } else {
                // Last cards: give remaining cards to players
                self.finish_draft_round(draft_type)?;
                Ok(true) // Draft round complete
            }
        } else {
            Ok(false) // Still waiting for other players
        }
    }

    /// Finish the current draft round by giving remaining cards to players
    fn finish_draft_round(&mut self, draft_type: DraftType) -> Result<(), String> {
        let direction = self.get_pass_direction(draft_type);

        // First, collect all the remaining cards we need to transfer
        let mut transfers: Vec<(PlayerId, Vec<String>)> = Vec::new();

        for player in &self.players {
            let source_player_id = match direction {
                PassDirection::After => {
                    // Taking from previous player
                    self.get_player_before(&player.id)
                }
                PassDirection::Before => {
                    // Taking from next player
                    self.get_player_after(&player.id)
                }
            };

            if let Some(source_id) = source_player_id {
                if let Some(source_player) = self.get_player(&source_id) {
                    // Collect remaining cards to transfer
                    transfers.push((player.id.clone(), source_player.draft_hand.clone()));
                }
            }
        }

        // Now apply the transfers
        for (target_id, cards) in transfers {
            if let Some(target_player) = self.get_player_mut(&target_id) {
                target_player.drafted_cards.extend(cards);
                target_player.needs_to_draft = false;
            }
        }

        // Clear draft hands
        for player in &mut self.players {
            player.draft_hand.clear();
        }

        Ok(())
    }

    /// Get the player before (previous) a given player
    fn get_player_before(&self, player_id: &PlayerId) -> Option<PlayerId> {
        let pos = self.players.iter().position(|p| p.id == *player_id)?;
        if pos == 0 {
            Some(self.players.last()?.id.clone())
        } else {
            Some(self.players[pos - 1].id.clone())
        }
    }

    /// Get the player after (next) a given player
    fn get_player_after(&self, player_id: &PlayerId) -> Option<PlayerId> {
        let pos = self.players.iter().position(|p| p.id == *player_id)?;
        if pos == self.players.len() - 1 {
            Some(self.players.first()?.id.clone())
        } else {
            Some(self.players[pos + 1].id.clone())
        }
    }

    /// End the current draft iteration and transition to next phase
    pub fn end_draft_iteration(&mut self, draft_type: DraftType) -> Result<(), String> {
        match draft_type {
            DraftType::Initial => {
                self.initial_draft_iteration += 1;
                self.draft_round = 1;

                match self.initial_draft_iteration {
                    2 => {
                        // Start second iteration
                        self.start_draft(DraftType::Initial)?;
                    }
                    3 => {
                        // Move drafted cards to dealt cards
                        for player in &mut self.players {
                            // For now, just move to hand (will be properly handled in research phase)
                            player.cards_in_hand.append(&mut player.drafted_cards);
                        }

                        // Check if prelude draft is enabled
                        // TODO: Check prelude draft variant flag
                        // For now, always transition to research phase
                        self.phase = crate::game::phase::Phase::Research;
                    }
                    _ => {
                        return Err("Invalid initial draft iteration".to_string());
                    }
                }
            }
            DraftType::Standard => {
                // Standard draft ends, transition to research phase
                for player in &mut self.players {
                    // Move drafted cards to hand for research phase
                    player.cards_in_hand.append(&mut player.drafted_cards);
                }
                self.phase = crate::game::phase::Phase::Research;
            }
            DraftType::Prelude => {
                // Prelude draft ends, transition to research phase
                // TODO: Store prelude cards separately
                for player in &mut self.players {
                    player.cards_in_hand.append(&mut player.drafted_cards);
                }
                self.phase = crate::game::phase::Phase::Research;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardType;

    #[test]
    fn test_draft_pass_direction() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Initial draft iteration 1: should pass after
        game.initial_draft_iteration = 1;
        assert_eq!(
            game.get_pass_direction(DraftType::Initial),
            PassDirection::After
        );

        // Initial draft iteration 2: should pass before
        game.initial_draft_iteration = 2;
        assert_eq!(
            game.get_pass_direction(DraftType::Initial),
            PassDirection::Before
        );

        // Standard draft: generation 1 (odd) = before, generation 2 (even) = after
        game.generation = 1;
        assert_eq!(
            game.get_pass_direction(DraftType::Standard),
            PassDirection::Before
        );

        game.generation = 2;
        assert_eq!(
            game.get_pass_direction(DraftType::Standard),
            PassDirection::After
        );
    }

    #[test]
    fn test_player_before_after() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        let p1_id = "p1".to_string();
        let p2_id = "p2".to_string();
        let p3_id = "p3".to_string();

        // Player 1: before = Player 3, after = Player 2
        assert_eq!(game.get_player_before(&p1_id), Some(p3_id.clone()));
        assert_eq!(game.get_player_after(&p1_id), Some(p2_id.clone()));

        // Player 2: before = Player 1, after = Player 3
        assert_eq!(game.get_player_before(&p2_id), Some(p1_id.clone()));
        assert_eq!(game.get_player_after(&p2_id), Some(p3_id.clone()));

        // Player 3: before = Player 2, after = Player 1
        assert_eq!(game.get_player_before(&p3_id), Some(p2_id.clone()));
        assert_eq!(game.get_player_after(&p3_id), Some(p1_id.clone()));
    }

    #[test]
    fn test_draft_selection() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start draft
        game.start_draft(DraftType::Initial).unwrap();

        // Player 1 should have 5 cards in draft hand
        let p1 = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(p1.draft_hand.len(), 5);
        assert!(p1.needs_to_draft);

        // Player 1 selects a card
        let selected = vec![p1.draft_hand[0].clone()];
        let done = game
            .process_draft_selection(&"p1".to_string(), selected, DraftType::Initial)
            .unwrap();
        assert!(!done); // Not done yet, Player 2 still needs to draft

        // Player 1 should have 4 cards left in draft hand and 1 in drafted
        let p1 = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(p1.draft_hand.len(), 4);
        assert_eq!(p1.drafted_cards.len(), 1);
        assert!(!p1.needs_to_draft);
    }

    #[test]
    fn test_standard_draft_initialization() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();

        // All players should have 4 cards in draft hand
        for player in &game.players {
            assert_eq!(player.draft_hand.len(), 4);
            assert!(player.needs_to_draft);
        }
    }

    #[test]
    fn test_standard_draft_pass_direction_generation_1() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Generation 1 (odd) should pass before (counter-clockwise)
        game.generation = 1;
        assert_eq!(
            game.get_pass_direction(DraftType::Standard),
            PassDirection::Before
        );
    }

    #[test]
    fn test_standard_draft_pass_direction_generation_2() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Generation 2 (even) should pass after (clockwise)
        game.generation = 2;
        assert_eq!(
            game.get_pass_direction(DraftType::Standard),
            PassDirection::After
        );
    }

    #[test]
    fn test_standard_draft_card_passing_after() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Set up draft hands manually
        game.players[0].draft_hand = vec!["card1".to_string(), "card2".to_string(), "card3".to_string()];
        game.players[1].draft_hand = vec!["card4".to_string(), "card5".to_string(), "card6".to_string()];
        game.players[2].draft_hand = vec!["card7".to_string(), "card8".to_string(), "card9".to_string()];

        // Set generation to 2 (even) so we pass after (clockwise)
        game.generation = 2;
        game.draft_round = 2; // Not first round, so we pass cards

        // Pass cards
        game.pass_draft_cards(DraftType::Standard).unwrap();

        // After passing clockwise: P1 gets P3's cards, P2 gets P1's cards, P3 gets P2's cards
        assert_eq!(game.players[0].draft_hand, vec!["card7", "card8", "card9"]);
        assert_eq!(game.players[1].draft_hand, vec!["card1", "card2", "card3"]);
        assert_eq!(game.players[2].draft_hand, vec!["card4", "card5", "card6"]);
    }

    #[test]
    fn test_standard_draft_card_passing_before() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Set up draft hands manually
        game.players[0].draft_hand = vec!["card1".to_string(), "card2".to_string(), "card3".to_string()];
        game.players[1].draft_hand = vec!["card4".to_string(), "card5".to_string(), "card6".to_string()];
        game.players[2].draft_hand = vec!["card7".to_string(), "card8".to_string(), "card9".to_string()];

        // Set generation to 1 (odd) so we pass before (counter-clockwise)
        game.generation = 1;
        game.draft_round = 2; // Not first round, so we pass cards

        // Pass cards
        game.pass_draft_cards(DraftType::Standard).unwrap();

        // After passing counter-clockwise: P1 gets P2's cards, P2 gets P3's cards, P3 gets P1's cards
        assert_eq!(game.players[0].draft_hand, vec!["card4", "card5", "card6"]);
        assert_eq!(game.players[1].draft_hand, vec!["card7", "card8", "card9"]);
        assert_eq!(game.players[2].draft_hand, vec!["card1", "card2", "card3"]);
    }

    #[test]
    fn test_standard_draft_full_round() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();

        // Both players should have 4 cards
        assert_eq!(game.players[0].draft_hand.len(), 4);
        assert_eq!(game.players[1].draft_hand.len(), 4);

        // Player 1 selects a card
        let p1_card = game.players[0].draft_hand[0].clone();
        let done = game
            .process_draft_selection(&"p1".to_string(), vec![p1_card.clone()], DraftType::Standard)
            .unwrap();
        assert!(!done); // Player 2 still needs to draft

        // Player 1 should have 3 cards left and 1 drafted
        assert_eq!(game.players[0].draft_hand.len(), 3);
        assert_eq!(game.players[0].drafted_cards.len(), 1);
        assert!(game.players[0].drafted_cards.contains(&p1_card));

        // Player 2 selects a card
        let p2_card = game.players[1].draft_hand[0].clone();
        let done = game
            .process_draft_selection(&"p2".to_string(), vec![p2_card.clone()], DraftType::Standard)
            .unwrap();
        assert!(!done); // More rounds to go (3 cards left per player)

        // Both players should have 3 cards left and 1 drafted
        assert_eq!(game.players[0].draft_hand.len(), 3);
        assert_eq!(game.players[1].draft_hand.len(), 3);
        assert_eq!(game.players[0].drafted_cards.len(), 1);
        assert_eq!(game.players[1].drafted_cards.len(), 1);
    }

    #[test]
    fn test_standard_draft_multiple_rounds() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();
        assert_eq!(game.draft_round, 1);

        // Round 1: Both players draft
        let p1_r1_card = game.players[0].draft_hand[0].clone();
        game.process_draft_selection(&"p1".to_string(), vec![p1_r1_card], DraftType::Standard).unwrap();
        
        let p2_r1_card = game.players[1].draft_hand[0].clone();
        game.process_draft_selection(&"p2".to_string(), vec![p2_r1_card], DraftType::Standard).unwrap();

        // Should have moved to round 2
        assert_eq!(game.draft_round, 2);
        assert_eq!(game.players[0].draft_hand.len(), 3);
        assert_eq!(game.players[1].draft_hand.len(), 3);

        // Round 2: Both players draft again
        let p1_r2_card = game.players[0].draft_hand[0].clone();
        game.process_draft_selection(&"p1".to_string(), vec![p1_r2_card], DraftType::Standard).unwrap();
        
        let p2_r2_card = game.players[1].draft_hand[0].clone();
        game.process_draft_selection(&"p2".to_string(), vec![p2_r2_card], DraftType::Standard).unwrap();

        // Should have moved to round 3
        assert_eq!(game.draft_round, 3);
        assert_eq!(game.players[0].draft_hand.len(), 2);
        assert_eq!(game.players[1].draft_hand.len(), 2);
    }

    #[test]
    fn test_standard_draft_completion() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();

        // Draft all 4 rounds
        for _round in 1..=4 {
            // Player 1 drafts
            let p1_card = game.players[0].draft_hand[0].clone();
            game.process_draft_selection(&"p1".to_string(), vec![p1_card], DraftType::Standard).unwrap();
            
            // Player 2 drafts
            let p2_card = game.players[1].draft_hand[0].clone();
            let done = game.process_draft_selection(&"p2".to_string(), vec![p2_card], DraftType::Standard).unwrap();
            
            if done {
                // Last round completed
                break;
            }
        }

        // After completion, all cards should be in drafted_cards
        // Each player should have 4 drafted cards (one from each round)
        assert_eq!(game.players[0].drafted_cards.len(), 4);
        assert_eq!(game.players[1].drafted_cards.len(), 4);
        
        // Draft hands should be empty
        assert!(game.players[0].draft_hand.is_empty());
        assert!(game.players[1].draft_hand.is_empty());
        
        // No one should need to draft
        assert!(!game.players[0].needs_to_draft);
        assert!(!game.players[1].needs_to_draft);
    }

    #[test]
    fn test_standard_draft_end_iteration() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Manually set up completed draft
        game.players[0].drafted_cards = vec!["card1".to_string(), "card2".to_string(), "card3".to_string(), "card4".to_string()];
        game.players[1].drafted_cards = vec!["card5".to_string(), "card6".to_string(), "card7".to_string(), "card8".to_string()];

        // End standard draft iteration
        game.end_draft_iteration(DraftType::Standard).unwrap();

        // Should transition to research phase
        assert_eq!(game.phase, crate::game::phase::Phase::Research);
        
        // Drafted cards should be moved to hand
        assert_eq!(game.players[0].cards_in_hand.len(), 4);
        assert_eq!(game.players[1].cards_in_hand.len(), 4);
        assert!(game.players[0].drafted_cards.is_empty());
        assert!(game.players[1].drafted_cards.is_empty());
    }

    #[test]
    fn test_standard_draft_invalid_selection() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();

        // Try to select wrong number of cards (should fail)
        let result = game.process_draft_selection(
            &"p1".to_string(),
            vec!["card1".to_string(), "card2".to_string()], // Should be 1, not 2
            DraftType::Standard,
        );
        assert!(result.is_err());

        // Try to select a card not in hand (should fail)
        let result = game.process_draft_selection(
            &"p1".to_string(),
            vec!["nonexistent_card".to_string()],
            DraftType::Standard,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_standard_draft_three_players() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start standard draft
        game.start_draft(DraftType::Standard).unwrap();

        // All three players should have 4 cards
        for player in &game.players {
            assert_eq!(player.draft_hand.len(), 4);
            assert!(player.needs_to_draft);
        }

        // All three players draft
        for i in 0..3 {
            let player_id = format!("p{}", i + 1);
            let card = game.get_player(&player_id).unwrap().draft_hand[0].clone();
            game.process_draft_selection(&player_id, vec![card], DraftType::Standard).unwrap();
        }

        // Should have moved to round 2
        assert_eq!(game.draft_round, 2);
        for player in &game.players {
            assert_eq!(player.draft_hand.len(), 3);
            assert_eq!(player.drafted_cards.len(), 1);
        }
    }
}

