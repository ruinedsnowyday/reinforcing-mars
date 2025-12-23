use crate::player::PlayerId;

/// Represents a milestone that can be claimed
pub trait Milestone {
    /// Get the milestone name/ID
    fn name(&self) -> &str;
    
    /// Check if a player can claim this milestone
    fn can_claim(&self, player_id: PlayerId) -> bool;
    
    /// Get the cost to claim this milestone (in M€)
    fn cost(&self) -> i32;
}

/// Tracks a claimed milestone
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClaimedMilestone {
    pub player_id: PlayerId,
    pub milestone_name: String,
}

/// Milestone implementation (simplified for Phase 1)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MilestoneData {
    pub name: String,
    pub cost: i32,
    // Requirements will be implemented in later phases
}

impl Milestone for MilestoneData {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_claim(&self, _player_id: PlayerId) -> bool {
        // Simplified for Phase 1 - full implementation in later phases
        true
    }

    fn cost(&self) -> i32 {
        self.cost
    }
}

