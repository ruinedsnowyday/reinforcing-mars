use crate::player::{Player, PlayerId};
use crate::game::phase::Phase;
use crate::game::global_params::GlobalParameters;
use crate::game::milestones::{MilestoneData, ClaimedMilestone};
use crate::game::awards::{AwardData, FundedAward};

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
    
    /// Global parameters
    pub global_parameters: GlobalParameters,
    
    /// RNG seed
    pub rng_seed: u64,
    
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
        corporate_era: bool,
        venus_next: bool,
        colonies: bool,
        prelude: bool,
        prelude2: bool,
        turmoil: bool,
        promos: bool,
    ) -> Self {
        let players: Vec<Player> = player_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                Player::new(format!("p{}", i + 1), name)
            })
            .collect();
        
        let solo_mode = players.len() == 1;
        let neutral_player = if solo_mode {
            Some(Player::new("neutral".to_string(), "Neutral".to_string()))
        } else {
            None
        };
        
        Self {
            id,
            players,
            phase: Phase::InitialDrafting,
            generation: 1,
            active_player_id: None,
            global_parameters: GlobalParameters::new(),
            rng_seed,
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
            false, false, false, false, false, false, false,
        );
        
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.generation, 1);
        assert_eq!(game.phase, Phase::InitialDrafting);
        assert!(!game.is_solo_mode());
    }

    #[test]
    fn test_solo_mode() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            false, false, false, false, false, false, false,
        );
        
        assert!(game.is_solo_mode());
        assert!(game.neutral_player.is_some());
    }
}

