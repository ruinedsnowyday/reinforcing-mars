/// Base game cards implementation
/// Phase 9, Group 1: Simple Automated Cards
use crate::cards::{Card, CardType, Behavior, ProductionChange, StockChange, GlobalParameterChange};
use crate::player::tags::Tag;
use crate::cards::card_registry::CardRegistry;
use crate::game::global_params::GlobalParameter;

/// Register all base game simple automated cards
pub fn register_base_game_automated_cards(registry: &mut CardRegistry) {
    // Power Plant - Gain 1 energy production
    registry.register(
        Card::new(
            "power_plant".to_string(),
            "Power Plant".to_string(),
            CardType::Automated,
        )
        .with_cost(4)
        .with_tags(vec![Tag::Building, Tag::Power])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                energy: Some(1),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Mining Area - Gain 1 steel production
    registry.register(
        Card::new(
            "mining_area".to_string(),
            "Mining Area".to_string(),
            CardType::Automated,
        )
        .with_cost(4)
        .with_tags(vec![Tag::Building])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                steel: Some(1),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Building Industries - Gain 2 steel production
    registry.register(
        Card::new(
            "building_industries".to_string(),
            "Building Industries".to_string(),
            CardType::Automated,
        )
        .with_cost(6)
        .with_tags(vec![Tag::Building])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                steel: Some(2),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Acquired Company - Gain 3 M€ production
    registry.register(
        Card::new(
            "acquired_company".to_string(),
            "Acquired Company".to_string(),
            CardType::Automated,
        )
        .with_cost(10)
        .with_tags(vec![Tag::Earth])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                megacredits: Some(3),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Insulation - Gain 2 heat production
    registry.register(
        Card::new(
            "insulation".to_string(),
            "Insulation".to_string(),
            CardType::Automated,
        )
        .with_cost(2)
        .with_tags(vec![Tag::Building])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                heat: Some(2),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Deep Well Heating - Gain 1 energy production
    registry.register(
        Card::new(
            "deep_well_heating".to_string(),
            "Deep Well Heating".to_string(),
            CardType::Automated,
        )
        .with_cost(13)
        .with_tags(vec![Tag::Building, Tag::Power])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                energy: Some(1),
                ..Default::default()
            }),
            ..Default::default()
        })
    );

    // Tectonic Stress Power - Gain 1 energy production
    registry.register(
        Card::new(
            "tectonic_stress_power".to_string(),
            "Tectonic Stress Power".to_string(),
            CardType::Automated,
        )
        .with_cost(18)
        .with_tags(vec![Tag::Building, Tag::Power])
        .with_behavior(Behavior {
            production: Some(ProductionChange {
                energy: Some(1),
                ..Default::default()
            }),
            ..Default::default()
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::game::Game;
    use crate::board::BoardType;
    use crate::player::Player;
    use crate::cards::card_play::CardPlay;
    use crate::actions::payment::Payment;

    #[test]
    fn test_power_plant_card() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let card = registry.get(&"power_plant".to_string()).unwrap();
        assert_eq!(card.name, "Power Plant");
        assert_eq!(card.get_cost(), 4);
        assert!(card.has_tag(Tag::Building));
        assert!(card.has_tag(Tag::Power));
        assert_eq!(card.card_type, CardType::Automated);
        assert!(card.behavior.is_some());
    }

    #[test]
    fn test_mining_area_card() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let card = registry.get(&"mining_area".to_string()).unwrap();
        assert_eq!(card.name, "Mining Area");
        assert_eq!(card.get_cost(), 4);
        assert!(card.has_tag(Tag::Building));
        assert_eq!(card.card_type, CardType::Automated);
    }

    #[test]
    fn test_building_industries_card() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let card = registry.get(&"building_industries".to_string()).unwrap();
        assert_eq!(card.name, "Building Industries");
        assert_eq!(card.get_cost(), 6);
        assert!(card.has_tag(Tag::Building));
        assert_eq!(card.card_type, CardType::Automated);
    }

    #[test]
    fn test_power_plant_effect() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        let mut player = game.players[0].clone();
        let initial_energy_prod = player.production.energy;
        
        let card = registry.get(&"power_plant".to_string()).unwrap().clone();
        player.add_card_to_hand(card.id.clone());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);
        
        // Play the card
        let payment = Payment::with_megacredits(4);
        CardPlay::play_card(&card, &mut player, &mut game, &payment).unwrap();
        
        // Check that energy production increased
        assert_eq!(player.production.energy, initial_energy_prod + 1);
        // Check that card is in played cards
        assert!(player.played_cards.contains(&card.id));
        // Check that card is not in hand
        assert!(!player.cards_in_hand.contains(&card.id));
    }

    #[test]
    fn test_mining_area_effect() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        let mut player = game.players[0].clone();
        let initial_steel_prod = player.production.steel;
        
        let card = registry.get(&"mining_area".to_string()).unwrap().clone();
        player.add_card_to_hand(card.id.clone());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);
        
        // Play the card
        let payment = Payment::with_megacredits(4);
        CardPlay::play_card(&card, &mut player, &mut game, &payment).unwrap();
        
        // Check that steel production increased
        assert_eq!(player.production.steel, initial_steel_prod + 1);
    }

    #[test]
    fn test_building_industries_effect() {
        let mut registry = CardRegistry::new();
        register_base_game_automated_cards(&mut registry);
        
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        let mut player = game.players[0].clone();
        let initial_steel_prod = player.production.steel;
        
        let card = registry.get(&"building_industries".to_string()).unwrap().clone();
        player.add_card_to_hand(card.id.clone());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);
        
        // Play the card
        let payment = Payment::with_megacredits(6);
        CardPlay::play_card(&card, &mut player, &mut game, &payment).unwrap();
        
        // Check that steel production increased by 2
        assert_eq!(player.production.steel, initial_steel_prod + 2);
    }
}

