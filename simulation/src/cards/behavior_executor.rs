use crate::cards::behavior::{Behavior, ProductionChange, StockChange, StandardResourceGain, GlobalParameterChange};
use crate::player::Player;
use crate::player::resources::Resource;
use crate::game::game::Game;
use crate::board::BoardType;

/// BehaviorExecutor interprets and executes card behaviors
/// This handles Tier 1 cards (80% of cards) that use declarative behavior definitions
pub struct BehaviorExecutor;

impl BehaviorExecutor {
    /// Execute a behavior for a player
    /// This applies the behavior effects to the player and game state
    pub fn execute(behavior: &Behavior, player: &mut Player, game: &mut Game) -> Result<(), String> {
        // Execute production changes
        if let Some(production) = &behavior.production {
            Self::apply_production_change(player, production)?;
        }

        // Execute stock changes
        if let Some(stock) = &behavior.stock {
            Self::apply_stock_change(player, stock)?;
        }

        // Execute standard resource gains
        if let Some(standard_resource) = &behavior.standard_resource {
            Self::apply_standard_resource_gain(player, standard_resource)?;
        }

        // Execute card resource gains (add resources to card)
        // Note: This will be fully implemented when we have card instances with resources
        if behavior.add_resources.is_some() {
            // Placeholder: Will be implemented when card resources are tracked
        }

        // Execute TR changes
        if let Some(tr_change) = behavior.tr {
            player.terraform_rating = (player.terraform_rating + tr_change).max(0);
        }

        // Execute global parameter changes
        if let Some(global) = &behavior.global {
            Self::apply_global_parameter_change(game, global)?;
        }

        // Execute tile placements
        // Note: This will be fully implemented when board system is complete
        if behavior.city.is_some() || behavior.greenery.is_some() || behavior.ocean.is_some() || behavior.tile.is_some() {
            // Placeholder: Will be implemented when board system is complete
        }

        // Execute draw cards
        if behavior.draw_cards.is_some() {
            // Placeholder: Will be implemented when deck system is complete
            // For now, we'll just note that cards should be drawn
        }

        // Execute titanium/steel value changes
        // Note: These affect payment conversion rates, will be implemented when payment system is enhanced
        if behavior.titanium_value.is_some() || behavior.steel_value.is_some() {
            // Placeholder: Will be implemented when payment system tracks these values
        }

        Ok(())
    }

    /// Apply production change to player
    fn apply_production_change(player: &mut Player, change: &ProductionChange) -> Result<(), String> {
        if let Some(mc) = change.megacredits {
            player.production.add(Resource::Megacredits, mc);
        }
        if let Some(steel) = change.steel {
            player.production.add(Resource::Steel, steel);
        }
        if let Some(titanium) = change.titanium {
            player.production.add(Resource::Titanium, titanium);
        }
        if let Some(plants) = change.plants {
            player.production.add(Resource::Plants, plants);
        }
        if let Some(energy) = change.energy {
            player.production.add(Resource::Energy, energy);
        }
        if let Some(heat) = change.heat {
            player.production.add(Resource::Heat, heat);
        }
        Ok(())
    }

    /// Apply stock change to player
    fn apply_stock_change(player: &mut Player, change: &StockChange) -> Result<(), String> {
        if let Some(mc) = change.megacredits {
            if mc > 0 {
                player.resources.add(Resource::Megacredits, mc as u32);
            } else {
                player.resources.subtract(Resource::Megacredits, (-mc) as u32);
            }
        }
        if let Some(steel) = change.steel {
            if steel > 0 {
                player.resources.add(Resource::Steel, steel as u32);
            } else {
                player.resources.subtract(Resource::Steel, (-steel) as u32);
            }
        }
        if let Some(titanium) = change.titanium {
            if titanium > 0 {
                player.resources.add(Resource::Titanium, titanium as u32);
            } else {
                player.resources.subtract(Resource::Titanium, (-titanium) as u32);
            }
        }
        if let Some(plants) = change.plants {
            if plants > 0 {
                player.resources.add(Resource::Plants, plants as u32);
            } else {
                player.resources.subtract(Resource::Plants, (-plants) as u32);
            }
        }
        if let Some(energy) = change.energy {
            if energy > 0 {
                player.resources.add(Resource::Energy, energy as u32);
            } else {
                player.resources.subtract(Resource::Energy, (-energy) as u32);
            }
        }
        if let Some(heat) = change.heat {
            if heat > 0 {
                player.resources.add(Resource::Heat, heat as u32);
            } else {
                player.resources.subtract(Resource::Heat, (-heat) as u32);
            }
        }
        Ok(())
    }

    /// Apply standard resource gain to player
    fn apply_standard_resource_gain(player: &mut Player, gain: &StandardResourceGain) -> Result<(), String> {
        player.resources.add(gain.resource, gain.amount);
        Ok(())
    }

    /// Apply global parameter change to game
    fn apply_global_parameter_change(game: &mut Game, change: &GlobalParameterChange) -> Result<(), String> {
        use crate::game::global_params::GlobalParameter;
        if change.steps > 0 {
            game.global_parameters.increase(change.parameter, change.steps as u32);
        } else if change.steps < 0 {
            game.global_parameters.decrease(change.parameter, (-change.steps) as u32);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::behavior::{Behavior, ProductionChange, StockChange, StandardResourceGain, GlobalParameterChange};
    use crate::game::global_params::GlobalParameter;

    #[test]
    fn test_execute_production_change() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        let mut behavior = Behavior::default();
        behavior.production = Some(ProductionChange {
            megacredits: Some(1),
            steel: Some(1),
            ..Default::default()
        });

        let initial_mc_prod = player.production.megacredits;
        let initial_steel_prod = player.production.steel;

        BehaviorExecutor::execute(&behavior, &mut player, &mut game).unwrap();

        assert_eq!(player.production.megacredits, initial_mc_prod + 1);
        assert_eq!(player.production.steel, initial_steel_prod + 1);
    }

    #[test]
    fn test_execute_stock_change() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        let mut behavior = Behavior::default();
        behavior.stock = Some(StockChange {
            megacredits: Some(5),
            steel: Some(3),
            ..Default::default()
        });

        let initial_mc = player.resources.megacredits;
        let initial_steel = player.resources.steel;

        BehaviorExecutor::execute(&behavior, &mut player, &mut game).unwrap();

        assert_eq!(player.resources.megacredits, initial_mc + 5);
        assert_eq!(player.resources.steel, initial_steel + 3);
    }

    #[test]
    fn test_execute_tr_change() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        let mut behavior = Behavior::default();
        behavior.tr = Some(1);

        let initial_tr = player.terraform_rating;

        BehaviorExecutor::execute(&behavior, &mut player, &mut game).unwrap();

        assert_eq!(player.terraform_rating, initial_tr + 1);
    }

    #[test]
    fn test_execute_global_parameter_change() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        let mut behavior = Behavior::default();
        behavior.global = Some(GlobalParameterChange {
            parameter: GlobalParameter::Temperature,
            steps: 1,
        });

        let initial_temp = game.global_parameters.get(GlobalParameter::Temperature);

        BehaviorExecutor::execute(&behavior, &mut player, &mut game).unwrap();

        // Temperature increases by 2 per step
        assert_eq!(game.global_parameters.get(GlobalParameter::Temperature), initial_temp + 2);
    }
}

