use crate::cards::Card;
use crate::player::Player;
use crate::game::game::Game;

/// Trait for card customization points
/// Tier 2 cards (15% of cards) implement these traits for specific customization
pub trait CardCustomization {
    /// Called when this card is played
    /// Returns an optional error message if the card cannot be played
    fn on_card_played(&self, player: &mut Player, game: &mut Game) -> Result<(), String> {
        // Default: no custom behavior
        Ok(())
    }

    /// Get card discount for playing another card
    /// Returns the discount amount in M€
    fn get_card_discount(&self, player: &Player, card: &Card) -> u32 {
        // Default: no discount
        0
    }

    /// Get custom victory points calculation
    /// Returns the victory points for this card
    fn get_victory_points(&self, player: &Player) -> i32 {
        // Default: use card's victory_points field
        0
    }
}

/// Trait for cards with actions (ACTIVE cards)
pub trait ActionCard {
    /// Check if the action can be activated
    fn can_act(&self, player: &Player, game: &Game) -> bool {
        // Default: action is always available
        true
    }

    /// Execute the action
    /// Returns an optional error message if the action cannot be executed
    fn action(&self, player: &mut Player, game: &mut Game) -> Result<(), String> {
        // Default: no action
        Ok(())
    }
}

/// Trait for card discounts
/// Used by cards that provide discounts to other cards
pub trait CardDiscount {
    /// Get the discount amount for a specific card
    fn get_discount(&self, player: &Player, card: &Card) -> u32 {
        // Default: no discount
        0
    }
}

/// Trait for card interactions
/// Used by cards that react to other cards being played
pub trait CardInteraction {
    /// Called when another card is played
    fn on_card_played(&self, owner: &mut Player, played_card: &Card, active_player: &Player, game: &mut Game) -> Result<(), String> {
        // Default: no interaction
        Ok(())
    }

    /// Called when a standard project is executed
    fn on_standard_project(&self, owner: &mut Player, _project_type: &str, _game: &mut Game) -> Result<(), String> {
        // Default: no interaction
        Ok(())
    }
}

/// Default implementation for Card
impl CardCustomization for Card {
    // Use default implementations
}

/// Default implementation for Card
impl ActionCard for Card {
    // Use default implementations
}

/// Default implementation for Card
impl CardDiscount for Card {
    // Use default implementations
}

/// Default implementation for Card
impl CardInteraction for Card {
    // Use default implementations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::CardType;
    use crate::board::BoardType;

    #[test]
    fn test_card_customization_default() {
        use crate::cards::CardCustomization;
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        );
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let mut player = game.players[0].clone();
        
        // Default implementation should succeed
        assert!(CardCustomization::on_card_played(&card, &mut player, &mut game).is_ok());
    }

    #[test]
    fn test_action_card_default() {
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Active,
        );
        let game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let player = game.players[0].clone();
        
        // Default implementation should allow action
        assert!(card.can_act(&player, &game));
    }

    #[test]
    fn test_card_discount_default() {
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        );
        let other_card = Card::new(
            "card2".to_string(),
            "Other Card".to_string(),
            CardType::Automated,
        );
        let game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        let player = game.players[0].clone();
        
        // Default implementation should return 0 discount
        assert_eq!(card.get_discount(&player, &other_card), 0);
    }
}

