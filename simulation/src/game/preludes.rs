use crate::game::game::Game;
use crate::player::PlayerId;

/// Preludes phase implementation
impl Game {
    /// Start the preludes phase
    /// Players will play their selected prelude cards (2 cards each)
    pub fn start_preludes_phase(&mut self) -> Result<(), String> {
        if !self.prelude {
            return Err("Prelude expansion not enabled".to_string());
        }

        // Verify all players have selected preludes
        for player in &self.players {
            if player.selected_preludes.len() != 2 {
                return Err(format!(
                    "Player {} has not selected 2 preludes",
                    player.id
                ));
            }
        }

        // Set active player to first player
        if let Some(first_player) = self.players.first() {
            self.active_player_id = Some(first_player.id.clone());
        }

        Ok(())
    }

    /// Check if a player has played all their preludes
    pub fn has_played_all_preludes(&self, player_id: &PlayerId) -> bool {
        let player = match self.get_player(player_id) {
            Some(p) => p,
            None => return false,
        };

        // Player should have 2 selected preludes, and both should be in played_cards
        player.selected_preludes.len() == 2
            && player
                .selected_preludes
                .iter()
                .all(|prelude_id| player.played_cards.contains(prelude_id))
    }

    /// Check if all players have played all their preludes
    pub fn all_players_played_preludes(&self) -> bool {
        self.players
            .iter()
            .all(|p| self.has_played_all_preludes(&p.id))
    }

    /// Play a prelude card for a player
    /// Returns error if prelude cannot be played
    pub fn play_prelude(
        &mut self,
        player_id: &PlayerId,
        prelude_id: String,
    ) -> Result<(), String> {
        if !self.prelude {
            return Err("Prelude expansion not enabled".to_string());
        }

        // Validate prelude before borrowing
        {
            let player = self
                .get_player(player_id)
                .ok_or_else(|| format!("Player {player_id} not found"))?;

            // Validate prelude is in selected preludes
            if !player.selected_preludes.contains(&prelude_id) {
                return Err(format!("Prelude {prelude_id} not in selected preludes"));
            }

            // Check if already played
            if player.played_cards.contains(&prelude_id) {
                return Err(format!("Prelude {prelude_id} already played"));
            }
        }

        // Execute prelude effects
        // For now, this is a placeholder - will be expanded when card system is implemented
        // TODO: Execute actual prelude effects based on card definition
        self.execute_prelude_effects(player_id, &prelude_id)?;

        // Add to played cards
        let player = self
            .get_player_mut(player_id)
            .ok_or_else(|| format!("Player {player_id} not found"))?;
        player.played_cards.push(prelude_id);

        Ok(())
    }

    /// Execute prelude effects
    /// This is a placeholder that will be expanded when the card system is implemented
    fn execute_prelude_effects(
        &mut self,
        _player_id: &PlayerId,
        _prelude_id: &str,
    ) -> Result<(), String> {
        // TODO: Implement actual prelude effects
        // For now, this is a placeholder
        // Prelude effects can include:
        // - Resource gains (M€, steel, titanium, plants, energy, heat)
        // - Production changes
        // - Global parameter increases
        // - Drawing cards
        // - Tile placement
        // - TR increases
        // - Other special effects

        Ok(())
    }

    /// Complete preludes phase and transition to action phase
    pub fn complete_preludes_phase(&mut self) -> Result<(), String> {
        if !self.all_players_played_preludes() {
            return Err("Not all players have played their preludes".to_string());
        }

        // Transition to action phase
        self.phase = crate::game::phase::Phase::Action;

        // Reset active player to first player for action phase
        if let Some(first_player) = self.players.first() {
            self.active_player_id = Some(first_player.id.clone());
        }

        Ok(())
    }

    /// Get the next player who needs to play a prelude
    /// Returns None if all players have played all preludes
    pub fn next_prelude_player(&self) -> Option<PlayerId> {
        // Find first player who hasn't played all preludes
        for player in &self.players {
            if !self.has_played_all_preludes(&player.id) {
                return Some(player.id.clone());
            }
        }
        None
    }

    /// Move to next player for prelude playing
    /// After a player plays a prelude, move to next player if current player is done
    pub fn advance_prelude_turn(&mut self) -> Result<(), String> {
        // Check if current player has played all preludes
        if let Some(active_id) = &self.active_player_id {
            if self.has_played_all_preludes(active_id) {
                // Move to next player who needs to play preludes
                if let Some(next_id) = self.next_prelude_player() {
                    self.active_player_id = Some(next_id);
                } else {
                    // All players have played all preludes
                    self.complete_preludes_phase()?;
                }
            }
        } else {
            // No active player, set to first player who needs to play
            if let Some(next_id) = self.next_prelude_player() {
                self.active_player_id = Some(next_id);
            } else {
                // All players have played all preludes
                self.complete_preludes_phase()?;
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
    fn test_start_preludes_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes for both players
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.players[1].selected_preludes = vec!["prelude3".to_string(), "prelude4".to_string()];

        // Start preludes phase
        game.start_preludes_phase().unwrap();

        // Should have active player set
        assert!(game.active_player_id.is_some());
    }

    #[test]
    fn test_start_preludes_phase_missing_preludes() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Player hasn't selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string()];

        // Should fail
        let result = game.start_preludes_phase();
        assert!(result.is_err());
    }

    #[test]
    fn test_play_prelude() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.start_preludes_phase().unwrap();

        // Play first prelude
        game.play_prelude(&"p1".to_string(), "prelude1".to_string())
            .unwrap();

        // Should be in played cards
        assert!(game.players[0].played_cards.contains(&"prelude1".to_string()));
        assert!(!game.has_played_all_preludes(&"p1".to_string()));

        // Play second prelude
        game.play_prelude(&"p1".to_string(), "prelude2".to_string())
            .unwrap();

        // Should have played all preludes
        assert!(game.has_played_all_preludes(&"p1".to_string()));
    }

    #[test]
    fn test_play_prelude_invalid() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.start_preludes_phase().unwrap();

        // Try to play invalid prelude
        let result = game.play_prelude(&"p1".to_string(), "invalid_prelude".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_play_prelude_already_played() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.start_preludes_phase().unwrap();

        // Play prelude
        game.play_prelude(&"p1".to_string(), "prelude1".to_string())
            .unwrap();

        // Try to play same prelude again
        let result = game.play_prelude(&"p1".to_string(), "prelude1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_all_players_played_preludes() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.players[1].selected_preludes = vec!["prelude3".to_string(), "prelude4".to_string()];
        game.start_preludes_phase().unwrap();

        // Initially, not all players have played
        assert!(!game.all_players_played_preludes());

        // Player 1 plays both preludes
        game.play_prelude(&"p1".to_string(), "prelude1".to_string())
            .unwrap();
        game.play_prelude(&"p1".to_string(), "prelude2".to_string())
            .unwrap();

        // Still not all (Player 2 hasn't played)
        assert!(!game.all_players_played_preludes());

        // Player 2 plays both preludes
        game.play_prelude(&"p2".to_string(), "prelude3".to_string())
            .unwrap();
        game.play_prelude(&"p2".to_string(), "prelude4".to_string())
            .unwrap();

        // Now all players have played
        assert!(game.all_players_played_preludes());
    }

    #[test]
    fn test_complete_preludes_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.start_preludes_phase().unwrap();

        // Play both preludes
        game.play_prelude(&"p1".to_string(), "prelude1".to_string())
            .unwrap();
        game.play_prelude(&"p1".to_string(), "prelude2".to_string())
            .unwrap();

        // Complete preludes phase
        game.complete_preludes_phase().unwrap();

        // Should transition to action phase
        assert_eq!(game.phase, crate::game::phase::Phase::Action);
    }

    #[test]
    fn test_advance_prelude_turn() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, // prelude enabled
        );

        // Set up selected preludes
        game.players[0].selected_preludes = vec!["prelude1".to_string(), "prelude2".to_string()];
        game.players[1].selected_preludes = vec!["prelude3".to_string(), "prelude4".to_string()];
        game.start_preludes_phase().unwrap();

        // Initially, Player 1 is active
        assert_eq!(game.active_player_id, Some("p1".to_string()));

        // Player 1 plays first prelude
        game.play_prelude(&"p1".to_string(), "prelude1".to_string())
            .unwrap();
        game.advance_prelude_turn().unwrap();
        // Still Player 1 (hasn't played second prelude)
        assert_eq!(game.active_player_id, Some("p1".to_string()));

        // Player 1 plays second prelude
        game.play_prelude(&"p1".to_string(), "prelude2".to_string())
            .unwrap();
        game.advance_prelude_turn().unwrap();
        // Should move to Player 2
        assert_eq!(game.active_player_id, Some("p2".to_string()));

        // Player 2 plays both preludes
        game.play_prelude(&"p2".to_string(), "prelude3".to_string())
            .unwrap();
        game.play_prelude(&"p2".to_string(), "prelude4".to_string())
            .unwrap();
        game.advance_prelude_turn().unwrap();
        // Should transition to action phase
        assert_eq!(game.phase, crate::game::phase::Phase::Action);
    }
}

