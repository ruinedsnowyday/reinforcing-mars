use crate::cards::CardId;
use crate::actions::payment::Payment;

/// Milestone ID type
pub type MilestoneId = String;

/// Award ID type
pub type AwardId = String;

/// Standard project types
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StandardProjectType {
    /// Sell Patents: Select cards from hand, discard them, gain M€
    SellPatents,
    /// Power Plant: Gain energy production
    PowerPlant,
    /// Asteroid: Raise temperature, remove plants
    Asteroid,
    /// Aquifer: Place ocean tile
    Aquifer,
    /// Greenery: Place greenery tile, raise oxygen
    Greenery,
    /// City: Place city tile
    City,
}

/// Action enum - represents all actions a player can take
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Action {
    /// Play a card from hand
    PlayCard {
        /// Card ID to play
        card_id: CardId,
        /// Payment for the card
        payment: Payment,
    },
    /// Execute a standard project
    StandardProject {
        /// Type of standard project
        project_type: StandardProjectType,
        /// Payment for the project
        payment: Payment,
        /// Additional parameters (e.g., card IDs for Sell Patents)
        params: StandardProjectParams,
    },
    /// Pass (end turn)
    Pass,
    /// Convert Plants: Spend 8 plants to place 1 greenery tile (raises oxygen)
    ConvertPlants,
    /// Convert Heat: Spend 8 heat to raise TR by 1
    ConvertHeat,
    /// Fund an award
    FundAward {
        /// Award ID
        award_id: AwardId,
        /// Payment for funding
        payment: Payment,
    },
    /// Claim a milestone
    ClaimMilestone {
        /// Milestone ID
        milestone_id: MilestoneId,
        /// Payment for claiming
        payment: Payment,
    },
}

/// Additional parameters for standard projects
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct StandardProjectParams {
    /// Card IDs for Sell Patents (cards to discard)
    pub card_ids: Vec<CardId>,
}

impl Action {
    /// Check if this action is a pass action
    pub fn is_pass(&self) -> bool {
        matches!(self, Action::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_is_pass() {
        assert!(Action::Pass.is_pass());
        assert!(!Action::ConvertPlants.is_pass());
    }

    #[test]
    fn test_standard_project_params() {
        let params = StandardProjectParams {
            card_ids: vec!["card1".to_string(), "card2".to_string()],
        };
        assert_eq!(params.card_ids.len(), 2);
    }
}

