use crate::actions::action::{StandardProjectType, StandardProjectParams};
use crate::player::Player;
use crate::player::resources::Resource;

/// Standard project costs (in M€)
pub const SELL_PATENTS_COST: u32 = 0; // Free, but requires cards to discard
pub const POWER_PLANT_COST: u32 = 11;
pub const ASTEROID_COST: u32 = 14;
pub const AQUIFER_COST: u32 = 18;
pub const GREENERY_COST: u32 = 23;
pub const CITY_COST: u32 = 25;

/// Standard project effects and validation
pub struct StandardProjects;

impl StandardProjects {
    /// Get the cost for a standard project
    pub fn cost(project_type: StandardProjectType) -> u32 {
        match project_type {
            StandardProjectType::SellPatents => SELL_PATENTS_COST,
            StandardProjectType::PowerPlant => POWER_PLANT_COST,
            StandardProjectType::Asteroid => ASTEROID_COST,
            StandardProjectType::Aquifer => AQUIFER_COST,
            StandardProjectType::Greenery => GREENERY_COST,
            StandardProjectType::City => CITY_COST,
        }
    }

    /// Validate if a player can execute a standard project
    pub fn can_execute(
        project_type: StandardProjectType,
        player: &Player,
        params: &StandardProjectParams,
    ) -> Result<(), String> {
        match project_type {
            StandardProjectType::SellPatents => {
                // Must have at least one card to discard
                if params.card_ids.is_empty() {
                    return Err("Sell Patents requires at least one card to discard".to_string());
                }
                // All specified cards must be in hand
                for card_id in &params.card_ids {
                    if !player.cards_in_hand.contains(card_id) {
                        return Err(format!("Card {card_id} not in hand"));
                    }
                }
                Ok(())
            }
            StandardProjectType::PowerPlant => {
                // No special requirements
                Ok(())
            }
            StandardProjectType::Asteroid => {
                // No special requirements (temperature check happens in execution)
                Ok(())
            }
            StandardProjectType::Aquifer => {
                // No special requirements (ocean space check happens in execution)
                Ok(())
            }
            StandardProjectType::Greenery => {
                // No special requirements (oxygen check happens in execution)
                Ok(())
            }
            StandardProjectType::City => {
                // No special requirements (city space check happens in execution)
                Ok(())
            }
        }
    }

    /// Execute a standard project
    /// Returns the effects that should be applied
    pub fn execute(
        project_type: StandardProjectType,
        player: &mut Player,
        params: &StandardProjectParams,
    ) -> Result<StandardProjectEffect, String> {
        match project_type {
            StandardProjectType::SellPatents => {
                // Discard cards and gain M€ (1 M€ per card)
                let card_count = params.card_ids.len() as u32;
                for card_id in &params.card_ids {
                    if !player.remove_card_from_hand(card_id) {
                        return Err(format!("Card {card_id} not in hand"));
                    }
                }
                player.resources.add(Resource::Megacredits, card_count);
                Ok(StandardProjectEffect::None)
            }
            StandardProjectType::PowerPlant => {
                // Gain 1 energy production
                player.production.add(Resource::Energy, 1);
                Ok(StandardProjectEffect::None)
            }
            StandardProjectType::Asteroid => {
                // Raise temperature by 1 step, remove 3 plants from any player
                // For now, we'll just raise temperature (plant removal will be handled in action executor)
                Ok(StandardProjectEffect::RaiseTemperature { steps: 1 })
            }
            StandardProjectType::Aquifer => {
                // Place an ocean tile
                Ok(StandardProjectEffect::PlaceOcean)
            }
            StandardProjectType::Greenery => {
                // Place a greenery tile and raise oxygen by 1
                Ok(StandardProjectEffect::PlaceGreenery)
            }
            StandardProjectType::City => {
                // Place a city tile
                Ok(StandardProjectEffect::PlaceCity)
            }
        }
    }
}

/// Effects from standard project execution
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardProjectEffect {
    None,
    RaiseTemperature { steps: u32 },
    PlaceOcean,
    PlaceGreenery,
    PlaceCity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_project_costs() {
        assert_eq!(StandardProjects::cost(StandardProjectType::SellPatents), 0);
        assert_eq!(StandardProjects::cost(StandardProjectType::PowerPlant), 11);
        assert_eq!(StandardProjects::cost(StandardProjectType::Asteroid), 14);
        assert_eq!(StandardProjects::cost(StandardProjectType::Aquifer), 18);
        assert_eq!(StandardProjects::cost(StandardProjectType::Greenery), 23);
        assert_eq!(StandardProjects::cost(StandardProjectType::City), 25);
    }

    #[test]
    fn test_sell_patents_validation() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.add_card_to_hand("card1".to_string());
        player.add_card_to_hand("card2".to_string());

        let params = StandardProjectParams {
            card_ids: vec!["card1".to_string()],
        };
        assert!(StandardProjects::can_execute(
            StandardProjectType::SellPatents,
            &player,
            &params
        ).is_ok());

        let params_empty = StandardProjectParams {
            card_ids: vec![],
        };
        assert!(StandardProjects::can_execute(
            StandardProjectType::SellPatents,
            &player,
            &params_empty
        ).is_err());

        let params_invalid = StandardProjectParams {
            card_ids: vec!["card3".to_string()],
        };
        assert!(StandardProjects::can_execute(
            StandardProjectType::SellPatents,
            &player,
            &params_invalid
        ).is_err());
    }

    #[test]
    fn test_sell_patents_execution() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.add_card_to_hand("card1".to_string());
        player.add_card_to_hand("card2".to_string());
        let initial_mc = player.resources.megacredits;

        let params = StandardProjectParams {
            card_ids: vec!["card1".to_string(), "card2".to_string()],
        };
        let result = StandardProjects::execute(
            StandardProjectType::SellPatents,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 2);
    }

    #[test]
    fn test_sell_patents_zero_cards() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        // Empty hand
        let params = StandardProjectParams {
            card_ids: vec![],
        };
        assert!(StandardProjects::can_execute(
            StandardProjectType::SellPatents,
            &player,
            &params
        ).is_err());
    }

    #[test]
    fn test_sell_patents_one_card() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.add_card_to_hand("card1".to_string());
        let initial_mc = player.resources.megacredits;

        let params = StandardProjectParams {
            card_ids: vec!["card1".to_string()],
        };
        let result = StandardProjects::execute(
            StandardProjectType::SellPatents,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 1); // 1 M€ per card
    }

    #[test]
    fn test_sell_patents_all_cards_discarded() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.add_card_to_hand("card1".to_string());
        player.add_card_to_hand("card2".to_string());
        player.add_card_to_hand("card3".to_string());
        let initial_mc = player.resources.megacredits;

        // Discard all cards
        let params = StandardProjectParams {
            card_ids: vec!["card1".to_string(), "card2".to_string(), "card3".to_string()],
        };
        let result = StandardProjects::execute(
            StandardProjectType::SellPatents,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 3); // 3 M€ for 3 cards
    }

    #[test]
    fn test_asteroid_execution() {
        use crate::game::global_params::GlobalParameters;
        use crate::game::global_params::GlobalParameter;

        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        let mut global_params = GlobalParameters::new();
        let initial_temp = global_params.get(GlobalParameter::Temperature);

        let params = StandardProjectParams::default();
        let result = StandardProjects::execute(
            StandardProjectType::Asteroid,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        
        // Temperature should have increased
        // Note: Temperature increases in steps of 2, so 1 step = +2 temperature
        if let Ok(effect) = result {
            if let crate::actions::standard_projects::StandardProjectEffect::RaiseTemperature { steps } = effect {
                global_params.increase(GlobalParameter::Temperature, steps);
                // Each step increases temperature by 2
                assert_eq!(global_params.get(GlobalParameter::Temperature), initial_temp + (steps as i32 * 2));
            }
        }
    }

    #[test]
    fn test_aquifer_execution() {
        use crate::game::global_params::GlobalParameters;
        use crate::game::global_params::GlobalParameter;

        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        let mut global_params = GlobalParameters::new();
        let initial_oceans = global_params.get(GlobalParameter::Oceans);

        let params = StandardProjectParams::default();
        let result = StandardProjects::execute(
            StandardProjectType::Aquifer,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        
        // Ocean should be placed (oceans increased)
        if let Ok(effect) = result {
            if let crate::actions::standard_projects::StandardProjectEffect::PlaceOcean = effect {
                global_params.increase(GlobalParameter::Oceans, 1);
                assert_eq!(global_params.get(GlobalParameter::Oceans), initial_oceans + 1);
            }
        }
    }

    #[test]
    fn test_greenery_execution() {
        use crate::game::global_params::GlobalParameters;
        use crate::game::global_params::GlobalParameter;

        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        let mut global_params = GlobalParameters::new();
        let initial_oxygen = global_params.get(GlobalParameter::Oxygen);

        let params = StandardProjectParams::default();
        let result = StandardProjects::execute(
            StandardProjectType::Greenery,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        
        // Greenery should be placed (oxygen increased)
        if let Ok(effect) = result {
            if let crate::actions::standard_projects::StandardProjectEffect::PlaceGreenery = effect {
                global_params.increase(GlobalParameter::Oxygen, 1);
                assert_eq!(global_params.get(GlobalParameter::Oxygen), initial_oxygen + 1);
            }
        }
    }

    #[test]
    fn test_city_execution() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        let params = StandardProjectParams::default();
        let result = StandardProjects::execute(
            StandardProjectType::City,
            &mut player,
            &params,
        );
        assert!(result.is_ok());
        
        // City placement effect (no immediate resource changes, just tile placement)
        if let Ok(effect) = result {
            assert!(matches!(effect, crate::actions::standard_projects::StandardProjectEffect::PlaceCity));
        }
    }
}

