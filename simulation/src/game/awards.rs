use crate::player::PlayerId;

/// Represents an award that can be funded
pub trait Award {
    /// Get the award name/ID
    fn name(&self) -> &str;
    
    /// Get the cost to fund this award (in M€)
    fn funding_cost(&self) -> i32;
    
    /// Calculate the score for a player (for determining award winner)
    fn calculate_score(&self, player_id: PlayerId) -> i32;
}

/// Tracks a funded award
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FundedAward {
    pub player_id: PlayerId,
    pub award_name: String,
}

/// Award implementation (simplified for Phase 1)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AwardData {
    pub name: String,
    pub funding_cost: i32,
    // Scoring criteria will be implemented in later phases
}

impl Award for AwardData {
    fn name(&self) -> &str {
        &self.name
    }

    fn funding_cost(&self) -> i32 {
        self.funding_cost
    }

    fn calculate_score(&self, _player_id: PlayerId) -> i32 {
        // Simplified for Phase 1 - full implementation in later phases
        0
    }
}

