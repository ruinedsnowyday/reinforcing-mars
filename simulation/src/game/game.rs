use crate::player::{Player, PlayerId};
use crate::game::phase::Phase;
use crate::game::global_params::GlobalParameters;
use crate::game::milestones::{MilestoneData, ClaimedMilestone};
use crate::game::awards::{AwardData, FundedAward};
use crate::board::{Board, BoardType};
use crate::utils::random::SeededRandom;

/// Game struct - tracks game state
/// This is a skeleton implementation for Phase 1
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Game {
    /// Game ID
    pub id: String,
    
    /// Players in the game
    pub players: Vec<Player>,
    
    /// Current phase
    pub phase: Phase,
    
    /// Current generation (starts at 1)
    pub generation: u32,
    
    /// Active player ID
    pub active_player_id: Option<PlayerId>,
    
    /// Players who have passed in the current action phase
    pub passed_players: Vec<PlayerId>,
    
    /// Global parameters
    pub global_parameters: GlobalParameters,
    
    /// Board
    pub board: Board,
    
    /// RNG seed
    pub rng_seed: u64,
    
    /// Seeded random number generator
    #[serde(skip)]
    pub rng: SeededRandom,
    
    /// Expansion flags
    pub corporate_era: bool,
    pub venus_next: bool,
    pub colonies: bool,
    pub prelude: bool,
    pub prelude2: bool,
    pub turmoil: bool,
    pub promos: bool,
    
    /// Milestones
    pub milestones: Vec<MilestoneData>,
    pub claimed_milestones: Vec<ClaimedMilestone>,
    
    /// Awards
    pub awards: Vec<AwardData>,
    pub funded_awards: Vec<FundedAward>,
    
    /// Solo mode flag
    pub solo_mode: bool,
    
    /// Neutral player (for solo mode)
    pub neutral_player: Option<Player>,
}

impl Game {
    /// Create a new game
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        player_names: Vec<String>,
        rng_seed: u64,
        board_type: BoardType,
        corporate_era: bool,
        venus_next: bool,
        colonies: bool,
        prelude: bool,
        prelude2: bool,
        turmoil: bool,
        promos: bool,
    ) -> Self {
        let solo_mode = player_names.len() == 1;
        
        let players: Vec<Player> = player_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                let mut player = Player::new(format!("p{}", i + 1), name);
                
                // Solo mode: player starts with 14 TR instead of 20
                if solo_mode {
                    player.terraform_rating = 14;
                }
                
                player
            })
            .collect();
        let neutral_player = if solo_mode {
            Some(Player::new("neutral".to_string(), "Neutral".to_string()))
        } else {
            None
        };
        
        let board = Board::new(board_type);
        let rng = SeededRandom::new(rng_seed);
        
        // Set first player as active
        let active_player_id = players.first().map(|p| p.id.clone());
        
        Self {
            id,
            players,
            phase: Phase::InitialDrafting,
            generation: 1,
            active_player_id,
            passed_players: Vec::new(),
            global_parameters: GlobalParameters::new(),
            board,
            rng_seed,
            rng,
            corporate_era,
            venus_next,
            colonies,
            prelude,
            prelude2,
            turmoil,
            promos,
            milestones: Vec::new(),
            claimed_milestones: Vec::new(),
            awards: Vec::new(),
            funded_awards: Vec::new(),
            solo_mode,
            neutral_player,
        }
    }

    /// Get a player by ID
    pub fn get_player(&self, player_id: &PlayerId) -> Option<&Player> {
        self.players.iter().find(|p| p.id == *player_id)
    }

    /// Get a mutable player by ID
    pub fn get_player_mut(&mut self, player_id: &PlayerId) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == *player_id)
    }

    /// Check if Mars is fully terraformed
    pub fn is_fully_terraformed(&self) -> bool {
        self.global_parameters.is_fully_terraformed()
    }

    /// Check if game is in solo mode
    pub fn is_solo_mode(&self) -> bool {
        self.solo_mode
    }

    /// Transition to the next phase
    pub fn next_phase(&mut self) -> Result<(), String> {
        self.phase = self.phase.next().ok_or("Game has ended")?;
        Ok(())
    }

    /// Get the active player
    pub fn active_player(&self) -> Option<&Player> {
        self.active_player_id
            .as_ref()
            .and_then(|id| self.get_player(id))
    }

    /// Get the active player mutably
    pub fn active_player_mut(&mut self) -> Option<&mut Player> {
        let player_id = self.active_player_id.clone()?;
        self.get_player_mut(&player_id)
    }

    /// Move to the next player
    pub fn next_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        let current_index = self
            .active_player_id
            .as_ref()
            .and_then(|id| {
                self.players
                    .iter()
                    .position(|p| p.id == *id)
            })
            .unwrap_or(0);

        let next_index = (current_index + 1) % self.players.len();
        self.active_player_id = Some(self.players[next_index].id.clone());
    }

    /// Mark the current player as passed
    pub fn pass_player(&mut self) -> Result<(), String> {
        let player_id = self
            .active_player_id
            .as_ref()
            .ok_or("No active player")?;

        if !self.passed_players.contains(player_id) {
            self.passed_players.push(player_id.clone());
        }

        Ok(())
    }

    /// Check if all players have passed
    pub fn all_players_passed(&self) -> bool {
        self.passed_players.len() >= self.players.len()
    }

    /// Reset passed players (for new action phase)
    pub fn reset_passed_players(&mut self) {
        self.passed_players.clear();
    }

    /// Increment generation and reset for next generation
    pub fn increment_generation(&mut self) {
        self.generation += 1;
        // Reset player states for new generation
        // (e.g., reset passed flags, etc.)
        // This will be expanded as we add more player state
    }

    /// Execute production phase: add production to resources
    pub fn execute_production_phase(&mut self) {
        for player in &mut self.players {
            // Add production to resources
            let production = &player.production;
            let resources = &mut player.resources;
            let tr = player.terraform_rating;

            // Add megacredits: production + TR (TR is always positive)
            let mc_production = production.megacredits;
            if mc_production >= 0 {
                resources.add(
                    crate::player::resources::Resource::Megacredits,
                    (mc_production as u32) + (tr as u32),
                );
            } else {
                // Negative production: add TR first, then subtract
                resources.add(
                    crate::player::resources::Resource::Megacredits,
                    tr as u32,
                );
                resources.subtract(
                    crate::player::resources::Resource::Megacredits,
                    (-mc_production) as u32,
                );
            }

            // Add other production (all non-negative)
            resources.add(
                crate::player::resources::Resource::Steel,
                production.steel,
            );
            resources.add(
                crate::player::resources::Resource::Titanium,
                production.titanium,
            );
            resources.add(
                crate::player::resources::Resource::Plants,
                production.plants,
            );
            resources.add(
                crate::player::resources::Resource::Energy,
                production.energy,
            );
            resources.add(
                crate::player::resources::Resource::Heat,
                production.heat,
            );

            // Energy converts to heat at end of production
            let energy = resources.get(crate::player::resources::Resource::Energy);
            if energy > 0 {
                resources.add(
                    crate::player::resources::Resource::Heat,
                    energy,
                );
                resources.set(
                    crate::player::resources::Resource::Energy,
                    0,
                );
            }
        }

        // Handle neutral player production in solo mode
        if let Some(ref mut _neutral) = self.neutral_player {
            // Neutral player production logic (simplified for now)
            // Will be expanded in later phases
        }
    }

    /// Check win conditions
    pub fn check_win_conditions(&self) -> Option<WinCondition> {
        if self.solo_mode {
            // Solo mode: win if TR >= 63 OR all global parameters maxed
            if let Some(player) = self.players.first() {
                if player.terraform_rating >= 63 {
                    return Some(WinCondition::SoloTr63);
                }
            }
        }

        // Check if terraformed (all enabled global parameters maxed)
        let oceans_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oceans,
        ) >= crate::game::global_params::MAX_OCEANS as i32;
        let oxygen_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oxygen,
        ) >= crate::game::global_params::MAX_OXYGEN as i32;
        let temperature_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Temperature,
        ) >= crate::game::global_params::MAX_TEMPERATURE;
        let venus_maxed = if self.venus_next {
            self.global_parameters.get(
                crate::game::global_params::GlobalParameter::Venus,
            ) >= crate::game::global_params::MAX_VENUS as i32
        } else {
            true // Venus not required if expansion not enabled
        };

        if oceans_maxed && oxygen_maxed && temperature_maxed && venus_maxed {
            return Some(WinCondition::Terraformed);
        }

        None
    }

    /// Calculate victory points for all players
    /// Returns a vector of (player_id, victory_points) tuples
    pub fn calculate_victory_points(&self) -> Vec<(PlayerId, u32)> {
        self.players
            .iter()
            .map(|player| {
                // Basic VP calculation: TR + other sources
                // This will be expanded in later phases
                let vp = player.terraform_rating.max(0) as u32;

                // TODO: Add other VP sources (cards, milestones, awards, etc.)

                (player.id.clone(), vp)
            })
            .collect()
    }

    /// Determine the winner based on victory points
    /// Returns the player ID with highest VP, or None if tie
    /// Tie-breaker: highest TR
    pub fn determine_winner(&self) -> Option<PlayerId> {
        let vps = self.calculate_victory_points();
        if vps.is_empty() {
            return None;
        }

        // Find player with highest VP
        let (winner_id, winner_vp) = vps.iter().max_by_key(|(_, vp)| vp)?;

        // Check for ties
        let tied_players: Vec<_> = vps
            .iter()
            .filter(|(_, vp)| vp == winner_vp)
            .collect();

        if tied_players.len() == 1 {
            return Some(winner_id.clone());
        }

        // Tie-breaker: highest TR
        let winner = tied_players
            .iter()
            .max_by_key(|(id, _)| {
                self.get_player(id)
                    .map(|p| p.terraform_rating)
                    .unwrap_or(0)
            })?;

        Some(winner.0.clone())
    }
}

/// Win condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinCondition {
    /// Solo mode: player reached TR 63
    SoloTr63,
    /// All global parameters maxed (multiplayer or solo)
    Terraformed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );
        
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.generation, 1);
        assert_eq!(game.phase, Phase::InitialDrafting);
        assert!(!game.is_solo_mode());
        assert!(game.active_player_id.is_some());
    }

    #[test]
    fn test_solo_mode() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );
        
        assert!(game.is_solo_mode());
        assert!(game.neutral_player.is_some());
        // Solo mode: player should start with 14 TR
        assert_eq!(game.players[0].terraform_rating, 14);
    }

    #[test]
    fn test_phase_transitions() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Initial phase
        assert_eq!(game.phase, Phase::InitialDrafting);

        // Transition to Research
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Research);
    }

    #[test]
    fn test_next_player() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        let first_player_id = game.active_player_id.clone();
        assert!(first_player_id.is_some());

        // Move to next player
        game.next_player();
        assert_ne!(game.active_player_id, first_player_id);

        // Move again
        let second_player_id = game.active_player_id.clone();
        game.next_player();
        assert_ne!(game.active_player_id, second_player_id);
        assert_ne!(game.active_player_id, first_player_id);

        // Should wrap around
        game.next_player();
        assert_eq!(game.active_player_id, first_player_id);
    }

    #[test]
    fn test_generation_increment() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        assert_eq!(game.generation, 1);
        game.increment_generation();
        assert_eq!(game.generation, 2);
        game.increment_generation();
        assert_eq!(game.generation, 3);
    }

    #[test]
    fn test_production_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = 5;
        player.production.steel = 2;
        player.production.energy = 3;
        player.terraform_rating = 20;

        // Execute production
        game.execute_production_phase();

        let player = game.players.first().unwrap();
        // Should have: 5 (production) + 20 (TR) = 25 megacredits
        assert_eq!(player.resources.megacredits, 25);
        assert_eq!(player.resources.steel, 2);
        // Energy should convert to heat
        assert_eq!(player.resources.energy, 0);
        assert_eq!(player.resources.heat, 3);
    }

    #[test]
    fn test_production_phase_negative_mc() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = -3; // Negative production
        player.terraform_rating = 20;

        game.execute_production_phase();

        let player = game.players.first().unwrap();
        // Should have: 20 (TR) - 3 (negative production) = 17 megacredits
        assert_eq!(player.resources.megacredits, 17);
    }

    #[test]
    fn test_pass_player() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        assert!(!game.all_players_passed());
        assert!(game.pass_player().is_ok());
        assert_eq!(game.passed_players.len(), 1);
        assert!(!game.all_players_passed());

        game.next_player();
        assert!(game.pass_player().is_ok());
        assert_eq!(game.passed_players.len(), 2);
        assert!(game.all_players_passed());
    }

    #[test]
    fn test_win_conditions() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        // Solo mode, TR < 63, not terraformed
        assert!(game.check_win_conditions().is_none());

        // Set TR to 63
        game.players[0].terraform_rating = 63;
        assert_eq!(
            game.check_win_conditions(),
            Some(WinCondition::SoloTr63)
        );

        // Reset and terraform
        game.players[0].terraform_rating = 20;
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oceans,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oxygen,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Temperature,
            100,
        );
        assert_eq!(
            game.check_win_conditions(),
            Some(WinCondition::Terraformed)
        );
    }

    #[test]
    fn test_victory_points() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false,
        );

        game.players[0].terraform_rating = 25;
        game.players[1].terraform_rating = 30;

        let vps = game.calculate_victory_points();
        assert_eq!(vps.len(), 2);
        assert!(vps.iter().any(|(id, vp)| id == "p1" && *vp == 25));
        assert!(vps.iter().any(|(id, vp)| id == "p2" && *vp == 30));

        // Player 2 should win
        assert_eq!(game.determine_winner(), Some("p2".to_string()));
    }
}

