use crate::game::game::Game;
use crate::player::PlayerId;

/// Research phase implementation
impl Game {
    /// Start the research phase
    /// For generation 1: initial research phase with corporation/prelude/project selection
    /// For subsequent generations: project card selection from drafted/dealt cards
    pub fn start_research_phase(&mut self) -> Result<(), String> {
        if self.generation == 1 {
            self.start_initial_research_phase()
        } else {
            self.start_standard_research_phase()
        }
    }

    /// Start initial research phase (generation 1)
    /// Deals corporation cards and sets up selection
    fn start_initial_research_phase(&mut self) -> Result<(), String> {
        // Deal corporation cards to each player (typically 2-3 cards)
        // For now, use placeholder card IDs
        // TODO: Integrate with actual corporation deck when implemented
        for player in &mut self.players {
            // Deal 2 corporation cards (can be 3 with certain variants)
            player.dealt_corporation_cards = (0..2)
                .map(|i| format!("corporation_card_{i}"))
                .collect();
        }

        Ok(())
    }

    /// Start standard research phase (generation 2+)
    /// If draft variant, cards come from drafting
    /// Otherwise, deal 4 cards
    fn start_standard_research_phase(&mut self) -> Result<(), String> {
        // For now, if cards are already in hand from drafting, use those
        // Otherwise, deal 4 cards
        // TODO: Check if draft variant is enabled and handle accordingly
        for player in &mut self.players {
            if player.cards_in_hand.is_empty() {
                // Deal 4 project cards
                // TODO: Integrate with actual card deck
                player.cards_in_hand = (0..4)
                    .map(|i| format!("project_card_{i}"))
                    .collect();
            }
        }

        Ok(())
    }

    /// Process corporation selection for a player
    /// Returns error if selection is invalid
    pub fn select_corporation(
        &mut self,
        player_id: &PlayerId,
        corporation_id: String,
    ) -> Result<(), String> {
        let player = self
            .get_player_mut(player_id)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        // Validate corporation is in dealt cards
        if !player.dealt_corporation_cards.contains(&corporation_id) {
            return Err(format!("Corporation {corporation_id} not in dealt cards"));
        }

        // Set selected corporation
        player.selected_corporation = Some(corporation_id.clone());

        // Remove from dealt cards
        player.dealt_corporation_cards.retain(|c| c != &corporation_id);

        // Apply corporation starting resources and production
        // For now, use default values (will be expanded when corporation system is implemented)
        // TODO: Apply actual corporation starting resources and production
        // Default: 42 M€ starting (will vary by corporation)
        player.resources.add(
            crate::player::resources::Resource::Megacredits,
            42,
        );

        Ok(())
    }

    /// Process prelude selection for a player
    /// Returns error if selection is invalid
    pub fn select_preludes(
        &mut self,
        player_id: &PlayerId,
        prelude_ids: Vec<String>,
    ) -> Result<(), String> {
        if !self.prelude {
            return Err("Prelude expansion not enabled".to_string());
        }

        if prelude_ids.len() != 2 {
            return Err("Must select exactly 2 prelude cards".to_string());
        }

        let player = self
            .get_player_mut(player_id)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        // Validate preludes are in dealt cards
        // For initial research, preludes come from drafted preludes
        // For now, check if they're in cards_in_hand (where drafted preludes would be)
        // TODO: Track prelude cards separately when prelude system is implemented
        for prelude_id in &prelude_ids {
            if !player.cards_in_hand.contains(prelude_id) {
                return Err(format!("Prelude {prelude_id} not available"));
            }
        }

        // Set selected preludes
        player.selected_preludes = prelude_ids;

        Ok(())
    }

    /// Process project card selection for a player
    /// Returns error if selection is invalid
    pub fn select_project_cards(
        &mut self,
        player_id: &PlayerId,
        card_ids: Vec<String>,
    ) -> Result<(), String> {
        if card_ids.len() > 10 {
            return Err("Cannot select more than 10 project cards".to_string());
        }

        // Check generation before borrowing
        let is_generation_1 = self.generation == 1;

        let player = self
            .get_player_mut(player_id)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        // For initial research, cards come from drafted project cards (in cards_in_hand)
        // For standard research, cards come from drafted/dealt cards
        // Validate all selected cards are available
        for card_id in &card_ids {
            if !player.cards_in_hand.contains(card_id) {
                return Err(format!("Card {card_id} not in hand"));
            }
        }

        // Move selected cards to hand (they're already there, but we mark them as selected)
        // Remove unselected cards
        let selected_set: std::collections::HashSet<_> = card_ids.iter().collect();
        player.cards_in_hand.retain(|c| selected_set.contains(c));

        // In initial research phase, player pays 3 M€ per card
        if is_generation_1 {
            let cost = (card_ids.len() as u32) * 3;
            player.resources.subtract(
                crate::player::resources::Resource::Megacredits,
                cost,
            );
        }

        Ok(())
    }

    /// Check if a player has completed research phase selection
    pub fn is_research_phase_complete(&self, player_id: &PlayerId) -> bool {
        let player = match self.get_player(player_id) {
            Some(p) => p,
            None => return false,
        };

        if self.generation == 1 {
            // Initial research: need corporation, preludes (if enabled), and project cards
            if player.selected_corporation.is_none() {
                return false;
            }
            if self.prelude && player.selected_preludes.len() != 2 {
                return false;
            }
            // Project cards are optional (0-10), so we don't check them
            true
        } else {
            // Standard research: just project card selection (optional)
            true
        }
    }

    /// Check if all players have completed research phase
    pub fn all_players_research_complete(&self) -> bool {
        self.players
            .iter()
            .all(|p| self.is_research_phase_complete(&p.id))
    }

    /// Complete research phase and transition to next phase
    pub fn complete_research_phase(&mut self) -> Result<(), String> {
        if !self.all_players_research_complete() {
            return Err("Not all players have completed research phase".to_string());
        }

        // Transition to next phase
        if self.generation == 1 {
            // Initial research: transition to PRELUDES (if enabled) or ACTION
            if self.prelude {
                self.phase = crate::game::phase::Phase::Preludes;
            } else {
                self.phase = crate::game::phase::Phase::Action;
            }
        } else {
            // Standard research: transition to ACTION
            self.phase = crate::game::phase::Phase::Action;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardType;

    #[test]
    fn test_initial_research_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start initial research phase
        game.start_research_phase().unwrap();

        // Both players should have dealt corporation cards
        assert_eq!(game.players[0].dealt_corporation_cards.len(), 2);
        assert_eq!(game.players[1].dealt_corporation_cards.len(), 2);
    }

    #[test]
    fn test_corporation_selection() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start initial research phase
        game.start_research_phase().unwrap();

        let corp_id = game.players[0].dealt_corporation_cards[0].clone();
        let initial_mc = game.players[0].resources.megacredits;

        // Select corporation
        game.select_corporation(&"p1".to_string(), corp_id.clone()).unwrap();

        // Corporation should be selected
        assert_eq!(game.players[0].selected_corporation, Some(corp_id.clone()));
        
        // Starting resources should be applied (42 M€)
        assert_eq!(game.players[0].resources.megacredits, initial_mc + 42);
        
        // Corporation should be removed from dealt cards
        assert!(!game.players[0].dealt_corporation_cards.contains(&corp_id));
    }

    #[test]
    fn test_corporation_selection_invalid() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start initial research phase
        game.start_research_phase().unwrap();

        // Try to select invalid corporation
        let result = game.select_corporation(&"p1".to_string(), "invalid_corp".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_prelude_selection() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Add prelude cards to hand (simulating drafted preludes)
        game.players[0].cards_in_hand = vec![
            "prelude1".to_string(),
            "prelude2".to_string(),
            "prelude3".to_string(),
            "prelude4".to_string(),
        ];

        // Select 2 preludes
        game.select_preludes(
            &"p1".to_string(),
            vec!["prelude1".to_string(), "prelude2".to_string()],
        )
        .unwrap();

        assert_eq!(game.players[0].selected_preludes.len(), 2);
        assert!(game.players[0].selected_preludes.contains(&"prelude1".to_string()));
        assert!(game.players[0].selected_preludes.contains(&"prelude2".to_string()));
    }

    #[test]
    fn test_prelude_selection_wrong_count() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Try to select wrong number of preludes
        let result = game.select_preludes(&"p1".to_string(), vec!["prelude1".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_project_card_selection() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Add cards to hand (simulating drafted cards)
        game.players[0].cards_in_hand = vec![
            "card1".to_string(),
            "card2".to_string(),
            "card3".to_string(),
            "card4".to_string(),
            "card5".to_string(),
        ];

        // Give player some megacredits for generation 1 test
        game.players[0].resources.megacredits = 50;
        let initial_mc = game.players[0].resources.megacredits;

        // Select 3 cards
        game.select_project_cards(
            &"p1".to_string(),
            vec!["card1".to_string(), "card2".to_string(), "card3".to_string()],
        )
        .unwrap();

        // Should have 3 cards in hand
        assert_eq!(game.players[0].cards_in_hand.len(), 3);
        
        // In generation 1, should pay 3 M€ per card (9 total)
        if game.generation == 1 {
            assert_eq!(game.players[0].resources.megacredits, initial_mc - 9);
        }
    }

    #[test]
    fn test_project_card_selection_too_many() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Try to select more than 10 cards
        let result = game.select_project_cards(
            &"p1".to_string(),
            (0..11).map(|i| format!("card{i}")).collect(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_research_phase_completion() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Start initial research phase
        game.start_research_phase().unwrap();

        // Select corporation
        let corp_id = game.players[0].dealt_corporation_cards[0].clone();
        game.select_corporation(&"p1".to_string(), corp_id).unwrap();

        // Should be complete (no preludes in this test)
        assert!(game.is_research_phase_complete(&"p1".to_string()));
        assert!(game.all_players_research_complete());

        // Complete research phase
        game.complete_research_phase().unwrap();

        // Should transition to ACTION phase (no preludes)
        assert_eq!(game.phase, crate::game::phase::Phase::Action);
    }

    #[test]
    fn test_research_phase_with_preludes() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Start initial research phase
        game.start_research_phase().unwrap();

        // Add prelude cards
        game.players[0].cards_in_hand = vec![
            "prelude1".to_string(),
            "prelude2".to_string(),
            "prelude3".to_string(),
            "prelude4".to_string(),
        ];

        // Select corporation
        let corp_id = game.players[0].dealt_corporation_cards[0].clone();
        game.select_corporation(&"p1".to_string(), corp_id).unwrap();

        // Select preludes
        game.select_preludes(
            &"p1".to_string(),
            vec!["prelude1".to_string(), "prelude2".to_string()],
        )
        .unwrap();

        // Should be complete
        assert!(game.is_research_phase_complete(&"p1".to_string()));

        // Complete research phase
        game.complete_research_phase().unwrap();

        // Should transition to PRELUDES phase
        assert_eq!(game.phase, crate::game::phase::Phase::Preludes);
    }
}

