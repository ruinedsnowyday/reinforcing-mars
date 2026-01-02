use crate::actions::action::Action;
use crate::actions::payment::{Payment, PaymentMethod};
use crate::actions::standard_projects::{StandardProjects, StandardProjectEffect};
use crate::actions::standard_actions::StandardActions;
use crate::player::Player;
use crate::player::resources::Resource;
use crate::game::game::Game;
use crate::game::global_params::GlobalParameter;
use crate::game::awards::Award;
use crate::game::milestones::Milestone;

/// Action executor - validates and executes actions
pub struct ActionExecutor;

impl ActionExecutor {
    /// Validate if an action can be executed
    pub fn can_execute(action: &Action, game: &Game, player_id: &str) -> Result<(), String> {
        let player_id_string = player_id.to_string();
        let player = game.get_player(&player_id_string)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        match action {
            Action::PlayCard { card_id, payment } => {
                // Check if card is in hand
                if !player.cards_in_hand.contains(card_id) {
                    return Err(format!("Card {card_id} not in hand"));
                }
                // Validate payment (will be enhanced when we have card costs)
                Self::validate_payment(payment, player, false, false)?;
                Ok(())
            }
            Action::StandardProject { project_type, payment, params } => {
                // Validate project-specific requirements
                StandardProjects::can_execute(*project_type, player, params)?;
                // Validate payment
                let cost = StandardProjects::cost(*project_type);
                Self::validate_payment_cost(payment, player, cost, false, false)?;
                Ok(())
            }
            Action::Pass => {
                // Pass is always valid
                Ok(())
            }
            Action::ConvertPlants => {
                StandardActions::can_convert_plants(player)
            }
            Action::ConvertHeat => {
                StandardActions::can_convert_heat(player)
            }
            Action::FundAward { award_id, payment } => {
                // Find award
                let award = game.awards.iter()
                    .find(|a| a.name == *award_id)
                    .ok_or_else(|| format!("Award {award_id} not found"))?;
                
                // Check if already funded
                if game.funded_awards.iter().any(|fa| fa.award_name == *award_id) {
                    return Err(format!("Award {award_id} already funded"));
                }
                
                // Validate payment
                let cost = award.funding_cost() as u32;
                Self::validate_payment_cost(payment, player, cost, false, false)?;
                Ok(())
            }
            Action::ClaimMilestone { milestone_id, payment } => {
                // Find milestone
                let milestone = game.milestones.iter()
                    .find(|m| m.name == *milestone_id)
                    .ok_or_else(|| format!("Milestone {milestone_id} not found"))?;
                
                // Check if already claimed
                if game.claimed_milestones.iter().any(|cm| cm.milestone_name == *milestone_id) {
                    return Err(format!("Milestone {milestone_id} already claimed"));
                }
                
                // Check if player can claim (simplified for Phase 4)
                if !milestone.can_claim(player_id.to_string()) {
                    return Err(format!("Player cannot claim milestone {milestone_id}"));
                }
                
                // Validate payment
                let cost = milestone.cost() as u32;
                Self::validate_payment_cost(payment, player, cost, false, false)?;
                Ok(())
            }
        }
    }

    /// Execute an action
    pub fn execute(action: &Action, game: &mut Game, player_id: &str) -> Result<(), String> {
        // Validate first
        Self::can_execute(action, game, player_id)?;

        let player_id_string = player_id.to_string();
        let player = game.get_player_mut(&player_id_string)
            .ok_or_else(|| format!("Player {player_id} not found"))?;

        match action {
            Action::PlayCard { card_id, payment } => {
                // Deduct payment
                Self::apply_payment(payment, player, false, false)?;
                // Move card from hand to played
                if !player.remove_card_from_hand(card_id) {
                    return Err(format!("Card {card_id} not in hand"));
                }
                player.add_played_card(card_id.clone());
                // Card effects will be implemented in Phase 5
                Ok(())
            }
            Action::StandardProject { project_type, payment, params } => {
                // Deduct payment
                Self::apply_payment(payment, player, false, false)?;
                // Execute project
                let effect = StandardProjects::execute(*project_type, player, params)?;
                // Apply effects
                Self::apply_standard_project_effect(effect, game, player_id)?;
                Ok(())
            }
            Action::Pass => {
                // Pass is handled by pass_player() in game.rs
                Ok(())
            }
            Action::ConvertPlants => {
                StandardActions::convert_plants(player)?;
                // Place greenery and raise oxygen (simplified for Phase 4)
                // Full implementation will be in Phase 4 when we have tile placement
                // For now, just raise oxygen
                game.global_parameters.increase(GlobalParameter::Oxygen, 1);
                Ok(())
            }
            Action::ConvertHeat => {
                StandardActions::convert_heat(player)?;
                Ok(())
            }
            Action::FundAward { award_id, payment } => {
                // Deduct payment
                Self::apply_payment(payment, player, false, false)?;
                // Fund award
                game.funded_awards.push(crate::game::awards::FundedAward {
                    player_id: player_id_string.clone(),
                    award_name: award_id.clone(),
                });
                Ok(())
            }
            Action::ClaimMilestone { milestone_id, payment } => {
                // Deduct payment
                Self::apply_payment(payment, player, false, false)?;
                // Claim milestone
                game.claimed_milestones.push(crate::game::milestones::ClaimedMilestone {
                    player_id: player_id_string.clone(),
                    milestone_name: milestone_id.clone(),
                });
                Ok(())
            }
        }
    }

    /// Validate payment can be made
    pub(crate) fn validate_payment(
        payment: &Payment,
        player: &Player,
        is_building_tag: bool,
        is_space_tag: bool,
    ) -> Result<(), String> {
        // Check reserve units
        if player.resources.megacredits < payment.reserve.megacredits {
            return Err("Insufficient megacredits to maintain reserve".to_string());
        }
        if player.resources.get(Resource::Steel) < payment.reserve.steel {
            return Err("Insufficient steel to maintain reserve".to_string());
        }
        if player.resources.get(Resource::Titanium) < payment.reserve.titanium {
            return Err("Insufficient titanium to maintain reserve".to_string());
        }
        if player.resources.get(Resource::Heat) < payment.reserve.heat {
            return Err("Insufficient heat to maintain reserve".to_string());
        }
        if player.resources.get(Resource::Plants) < payment.reserve.plants {
            return Err("Insufficient plants to maintain reserve".to_string());
        }

        // Check payment methods
        for method in &payment.methods {
            match method {
                PaymentMethod::MegaCredits(amount) => {
                    let available = player.resources.megacredits.saturating_sub(payment.reserve.megacredits);
                    if available < *amount {
                        return Err(format!("Insufficient megacredits: need {amount}, have {available}"));
                    }
                }
                PaymentMethod::Steel(amount) => {
                    if !is_building_tag {
                        return Err("Steel can only be used for building tags".to_string());
                    }
                    let available = player.resources.get(Resource::Steel).saturating_sub(payment.reserve.steel);
                    if available < *amount {
                        return Err(format!("Insufficient steel: need {amount}, have {available}"));
                    }
                }
                PaymentMethod::Titanium(amount) => {
                    if !is_space_tag {
                        return Err("Titanium can only be used for space tags".to_string());
                    }
                    let available = player.resources.get(Resource::Titanium).saturating_sub(payment.reserve.titanium);
                    if available < *amount {
                        return Err(format!("Insufficient titanium: need {amount}, have {available}"));
                    }
                }
                PaymentMethod::Heat(amount) => {
                    // Heat conversion requires Helion corporation (not implemented yet)
                    let available = player.resources.get(Resource::Heat).saturating_sub(payment.reserve.heat);
                    if available < *amount {
                        return Err(format!("Insufficient heat: need {amount}, have {available}"));
                    }
                }
                PaymentMethod::Plants(amount) => {
                    if !is_building_tag {
                        return Err("Plants can only be used for building tags (with Martian Lumber Corp)".to_string());
                    }
                    let available = player.resources.get(Resource::Plants).saturating_sub(payment.reserve.plants);
                    if available < *amount {
                        return Err(format!("Insufficient plants: need {amount}, have {available}"));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validate payment cost
    fn validate_payment_cost(
        payment: &Payment,
        player: &Player,
        required_mc: u32,
        is_building_tag: bool,
        is_space_tag: bool,
    ) -> Result<(), String> {
        let total_paid = payment.total_cost_mc(is_building_tag, is_space_tag);
        if total_paid < required_mc {
            return Err(format!("Insufficient payment: need {required_mc} M€, paying {total_paid} M€"));
        }
        Self::validate_payment(payment, player, is_building_tag, is_space_tag)
    }

    /// Apply payment (deduct resources)
    fn apply_payment(
        payment: &Payment,
        player: &mut Player,
        is_building_tag: bool,
        is_space_tag: bool,
    ) -> Result<(), String> {
        // Validate first
        Self::validate_payment(payment, player, is_building_tag, is_space_tag)?;

        // Apply each payment method
        for method in &payment.methods {
            match method {
                PaymentMethod::MegaCredits(amount) => {
                    player.resources.subtract(Resource::Megacredits, *amount);
                }
                PaymentMethod::Steel(amount) => {
                    if is_building_tag {
                        player.resources.subtract(Resource::Steel, *amount);
                    }
                }
                PaymentMethod::Titanium(amount) => {
                    if is_space_tag {
                        player.resources.subtract(Resource::Titanium, *amount);
                    }
                }
                PaymentMethod::Heat(amount) => {
                    player.resources.subtract(Resource::Heat, *amount);
                }
                PaymentMethod::Plants(amount) => {
                    if is_building_tag {
                        player.resources.subtract(Resource::Plants, *amount);
                    }
                }
            }
        }

        Ok(())
    }

    /// Apply standard project effects
    fn apply_standard_project_effect(
        effect: StandardProjectEffect,
        game: &mut Game,
        _player_id: &str,
    ) -> Result<(), String> {
        match effect {
            StandardProjectEffect::None => Ok(()),
            StandardProjectEffect::RaiseTemperature { steps } => {
                game.global_parameters.increase(GlobalParameter::Temperature, steps);
                // TODO: Remove 3 plants from any player (will be implemented when we have player selection)
                Ok(())
            }
            StandardProjectEffect::PlaceOcean => {
                // TODO: Place ocean tile (will be implemented when we have tile placement)
                game.global_parameters.increase(GlobalParameter::Oceans, 1);
                Ok(())
            }
            StandardProjectEffect::PlaceGreenery => {
                // TODO: Place greenery tile (will be implemented when we have tile placement)
                game.global_parameters.increase(GlobalParameter::Oxygen, 1);
                Ok(())
            }
            StandardProjectEffect::PlaceCity => {
                // TODO: Place city tile (will be implemented when we have tile placement)
                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::payment::Payment;
    use crate::game::phase::Phase;

    #[test]
    fn test_validate_payment_insufficient_mc() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.resources.add(Resource::Megacredits, 5);
        
        let payment = Payment::with_megacredits(10);
        assert!(ActionExecutor::validate_payment(&payment, &player, false, false).is_err());
    }

    #[test]
    fn test_validate_payment_sufficient_mc() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.resources.add(Resource::Megacredits, 10);
        
        let payment = Payment::with_megacredits(10);
        assert!(ActionExecutor::validate_payment(&payment, &player, false, false).is_ok());
    }

    #[test]
    fn test_action_validation_play_card_not_in_hand() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        // Try to play a card that's not in hand
        let action = Action::PlayCard {
            card_id: "nonexistent".to_string(),
            payment: Payment::default(),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_err());
    }

    #[test]
    fn test_action_validation_insufficient_payment() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        // Player has only 5 M€, but Power Plant costs 11 M€
        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 5);

        let action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::PowerPlant,
            payment: Payment::with_megacredits(5), // Only 5 M€, need 11
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_err());
    }

    #[test]
    fn test_action_execution_resource_deductions() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 25);
        let initial_mc = player.resources.megacredits;

        // Execute City project (costs 25 M€)
        let action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::City,
            payment: Payment::with_megacredits(25),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(ActionExecutor::execute(&action, &mut game, "p1").is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        // Should have deducted 25 M€
        assert_eq!(player.resources.megacredits, initial_mc - 25);
    }

    #[test]
    fn test_action_execution_milestone_claiming() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;
        use crate::game::milestones::MilestoneData;

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add a milestone
        game.milestones.push(MilestoneData {
            name: "test_milestone".to_string(),
            cost: 8,
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 8);

        // Claim milestone
        let action = Action::ClaimMilestone {
            milestone_id: "test_milestone".to_string(),
            payment: Payment::with_megacredits(8),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_ok());
        assert!(ActionExecutor::execute(&action, &mut game, "p1").is_ok());

        // Verify milestone was claimed
        assert_eq!(game.claimed_milestones.len(), 1);
        assert_eq!(game.claimed_milestones[0].player_id, "p1");
        assert_eq!(game.claimed_milestones[0].milestone_name, "test_milestone");

        // Verify payment was deducted
        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
    }

    #[test]
    fn test_action_execution_award_funding() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;
        use crate::game::awards::AwardData;

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add an award
        game.awards.push(AwardData {
            name: "test_award".to_string(),
            funding_cost: 8,
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 8);

        // Fund award
        let action = Action::FundAward {
            award_id: "test_award".to_string(),
            payment: Payment::with_megacredits(8),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_ok());
        assert!(ActionExecutor::execute(&action, &mut game, "p1").is_ok());

        // Verify award was funded
        assert_eq!(game.funded_awards.len(), 1);
        assert_eq!(game.funded_awards[0].player_id, "p1");
        assert_eq!(game.funded_awards[0].award_name, "test_award");

        // Verify payment was deducted
        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
    }

    #[test]
    fn test_action_validation_milestone_already_claimed() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;
        use crate::game::milestones::{MilestoneData, ClaimedMilestone};

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add a milestone and mark it as claimed
        game.milestones.push(MilestoneData {
            name: "test_milestone".to_string(),
            cost: 8,
        });
        game.claimed_milestones.push(ClaimedMilestone {
            player_id: "p1".to_string(),
            milestone_name: "test_milestone".to_string(),
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 8);

        // Try to claim already-claimed milestone
        let action = Action::ClaimMilestone {
            milestone_id: "test_milestone".to_string(),
            payment: Payment::with_megacredits(8),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_err());
    }

    #[test]
    fn test_action_validation_award_already_funded() {
        use crate::game::game::Game;
        use crate::board::BoardType;
        use crate::actions::action::Action;
        use crate::game::awards::{AwardData, FundedAward};

        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add an award and mark it as funded
        game.awards.push(AwardData {
            name: "test_award".to_string(),
            funding_cost: 8,
        });
        game.funded_awards.push(FundedAward {
            player_id: "p1".to_string(),
            award_name: "test_award".to_string(),
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(Resource::Megacredits, 8);

        // Try to fund already-funded award
        let action = Action::FundAward {
            award_id: "test_award".to_string(),
            payment: Payment::with_megacredits(8),
        };
        assert!(ActionExecutor::can_execute(&action, &game, "p1").is_err());
    }
}

