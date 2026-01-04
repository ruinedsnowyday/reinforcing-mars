use crate::cards::{Card, CardRegistry, BehaviorExecutor, CardCustomization};
use crate::player::Player;
use crate::game::game::Game;
use crate::actions::payment::Payment;

/// Card play helper functions
pub struct CardPlay;

impl CardPlay {
    /// Play a card using the full card system
    /// This integrates with the card registry, behavior executor, and trait system
    pub fn play_card(
        card: &Card,
        player: &mut Player,
        game: &mut Game,
        payment: &Payment,
    ) -> Result<(), String> {
        // 1. Validate card is in hand
        if !player.cards_in_hand.contains(&card.id) {
            return Err(format!("Card {} not in hand", card.id));
        }

        // 2. Check card requirements
        if let Some(requirements) = &card.requirements {
            requirements.satisfies(player, game)?;
        }

        // 3. Validate payment covers card cost
        let card_cost = card.get_cost();
        let is_building_tag = card.has_tag(crate::player::tags::Tag::Building);
        let is_space_tag = card.has_tag(crate::player::tags::Tag::Space);
        let total_paid = payment.total_cost_mc(is_building_tag, is_space_tag);
        if total_paid < card_cost {
            return Err(format!("Insufficient payment: need {} M€, paying {} M€", card_cost, total_paid));
        }

        // 4. Apply payment (deduct resources)
        // Use validate_payment to check, then manually deduct
        crate::actions::action_executor::ActionExecutor::validate_payment(payment, player, is_building_tag, is_space_tag)?;
        // Apply payment manually
        for method in &payment.methods {
            match method {
                crate::actions::payment::PaymentMethod::MegaCredits(amount) => {
                    player.resources.subtract(crate::player::resources::Resource::Megacredits, *amount);
                }
                crate::actions::payment::PaymentMethod::Steel(amount) => {
                    player.resources.subtract(crate::player::resources::Resource::Steel, *amount);
                }
                crate::actions::payment::PaymentMethod::Titanium(amount) => {
                    player.resources.subtract(crate::player::resources::Resource::Titanium, *amount);
                }
                crate::actions::payment::PaymentMethod::Heat(amount) => {
                    player.resources.subtract(crate::player::resources::Resource::Heat, *amount);
                }
                crate::actions::payment::PaymentMethod::Plants(amount) => {
                    player.resources.subtract(crate::player::resources::Resource::Plants, *amount);
                }
            }
        }

        // 5. Move card from hand to played
        player.remove_card_from_hand(&card.id);
        player.add_played_card(card.id.clone());

        // 6. Add card tags to player
        for tag in &card.tags {
            player.tags.add(*tag, 1);
        }

        // 7. Execute card behavior (if present)
        if let Some(behavior) = &card.behavior {
            BehaviorExecutor::execute(behavior, player, game)?;
        }

        // 8. Call trait methods
        CardCustomization::on_card_played(card, player, game)?;

        Ok(())
    }

    /// Play a card by ID (looks up card in registry)
    pub fn play_card_by_id(
        card_id: &str,
        registry: &CardRegistry,
        player: &mut Player,
        game: &mut Game,
        payment: &Payment,
    ) -> Result<(), String> {
        let card_id_string = card_id.to_string();
        let card = registry.get(&card_id_string)
            .ok_or_else(|| format!("Card {} not found in registry", card_id))?;
        Self::play_card(card, player, game, payment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{CardType, CardRegistry};
    use crate::cards::behavior::{Behavior, ProductionChange};
    use crate::actions::payment::Payment;
    use crate::board::BoardType;

    #[test]
    fn test_play_card_basic() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        // Create a simple card
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_cost(5);

        // Add card to hand
        player.add_card_to_hand("card1".to_string());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);

        // Create payment
        let payment = Payment::with_megacredits(5);

        // Play card
        CardPlay::play_card(&card, &mut player, &mut game, &payment).unwrap();

        // Verify card moved to played
        assert!(!player.cards_in_hand.contains(&"card1".to_string()));
        assert!(player.played_cards.contains(&"card1".to_string()));
        
        // Verify payment deducted
        assert_eq!(player.resources.megacredits, 5);
    }

    #[test]
    fn test_play_card_with_behavior() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        // Create a card with behavior
        let mut behavior = Behavior::default();
        behavior.production = Some(ProductionChange {
            megacredits: Some(1),
            ..Default::default()
        });
        
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_cost(5)
        .with_behavior(behavior);

        // Add card to hand
        player.add_card_to_hand("card1".to_string());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);
        let initial_mc_prod = player.production.megacredits;

        // Create payment
        let payment = Payment::with_megacredits(5);

        // Play card
        CardPlay::play_card(&card, &mut player, &mut game, &payment).unwrap();

        // Verify production increased
        assert_eq!(player.production.megacredits, initial_mc_prod + 1);
    }

    #[test]
    fn test_play_card_by_id() {
        let mut registry = CardRegistry::new();
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_cost(5);
        registry.register(card);

        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        player.add_card_to_hand("card1".to_string());
        player.resources.add(crate::player::resources::Resource::Megacredits, 10);

        let payment = Payment::with_megacredits(5);

        CardPlay::play_card_by_id("card1", &registry, &mut player, &mut game, &payment).unwrap();

        assert!(!player.cards_in_hand.contains(&"card1".to_string()));
        assert!(player.played_cards.contains(&"card1".to_string()));
    }
}

