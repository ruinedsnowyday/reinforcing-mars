/// Card requirements system
/// Supports tag requirements, global parameter requirements, etc.
use crate::player::tags::Tag;
use crate::game::global_params::GlobalParameter;
use crate::player::Player;
use crate::cards::Card;

/// Requirement type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RequirementType {
    /// Tag requirement (e.g., "Requires 2 science tags")
    Tag { tag: Tag, count: u32 },
    /// Global parameter requirement (e.g., "Requires 4 ocean tiles", "Oxygen must be 9% or less")
    GlobalParameter { 
        parameter: GlobalParameter, 
        count: i32, 
        max: bool, // If true, this is a maximum requirement (e.g., "Oxygen must be 9% or less")
    },
}

/// Card requirements descriptor (what the card needs)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardRequirements {
    pub requirements: Vec<RequirementType>,
}

impl CardRequirements {
    pub fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    pub fn with_tag_requirement(mut self, tag: Tag, count: u32) -> Self {
        self.requirements.push(RequirementType::Tag { tag, count });
        self
    }

    pub fn with_global_parameter_requirement(mut self, parameter: GlobalParameter, count: i32, max: bool) -> Self {
        self.requirements.push(RequirementType::GlobalParameter { parameter, count, max });
        self
    }

    /// Check if a player satisfies all requirements
    pub fn satisfies(&self, player: &Player, game: &crate::game::game::Game) -> Result<(), String> {
        for requirement in &self.requirements {
            match requirement {
                RequirementType::Tag { tag, count } => {
                    let player_tag_count = player.tags.count(*tag, false);
                    if player_tag_count < *count {
                        return Err(format!(
                            "Requires {} {} tags, but player has {}",
                            count,
                            format!("{:?}", tag),
                            player_tag_count
                        ));
                    }
                }
                RequirementType::GlobalParameter { parameter, count, max } => {
                    let current_value = game.global_parameters.get(*parameter);
                    if *max {
                        // Maximum requirement (e.g., "Oxygen must be 9% or less")
                        if current_value > *count {
                            return Err(format!(
                                "Requires {} to be {} or less, but it is {}",
                                format!("{:?}", parameter),
                                count,
                                current_value
                            ));
                        }
                    } else {
                        // Minimum requirement (e.g., "Requires 4 ocean tiles")
                        if current_value < *count {
                            return Err(format!(
                                "Requires {} to be at least {}, but it is {}",
                                format!("{:?}", parameter),
                                count,
                                current_value
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for CardRequirements {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::game::Game;
    use crate::board::BoardType;
    use crate::cards::CardType;

    #[test]
    fn test_tag_requirement() {
        let game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let player = &game.players[0];
        let card = Card::new("test".to_string(), "Test".to_string(), CardType::Automated);

        // Player has no science tags
        let requirements = CardRequirements::new()
            .with_tag_requirement(Tag::Science, 2);
        assert!(requirements.satisfies(player, &game).is_err());

        // Add science tags to player
        let mut player = game.players[0].clone();
        player.tags.add(Tag::Science, 2);
        assert!(requirements.satisfies(&player, &game).is_ok());
    }

    #[test]
    fn test_global_parameter_requirement_minimum() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let player = &game.players[0];
        let card = Card::new("test".to_string(), "Test".to_string(), CardType::Automated);

        // Requires 4 oceans, but we have 0
        let requirements = CardRequirements::new()
            .with_global_parameter_requirement(GlobalParameter::Oceans, 4, false);
        assert!(requirements.satisfies(player, &game).is_err());

        // Add 4 oceans
        game.global_parameters.increase(GlobalParameter::Oceans, 4);
        assert!(requirements.satisfies(player, &game).is_ok());
    }

    #[test]
    fn test_global_parameter_requirement_maximum() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let player = &game.players[0];
        let card = Card::new("test".to_string(), "Test".to_string(), CardType::Automated);

        // Requires oxygen to be 9 or less, we have 0 (ok)
        let requirements = CardRequirements::new()
            .with_global_parameter_requirement(GlobalParameter::Oxygen, 9, true);
        assert!(requirements.satisfies(player, &game).is_ok());

        // Increase oxygen to 10 (fails requirement)
        game.global_parameters.increase(GlobalParameter::Oxygen, 10);
        assert!(requirements.satisfies(player, &game).is_err());
    }
}

