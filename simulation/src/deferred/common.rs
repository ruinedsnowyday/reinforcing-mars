use crate::deferred::{DeferredAction, DeferredActionResult, Priority};
use crate::deferred::deferred_action::SimpleDeferredAction;
use crate::player::PlayerId;
use crate::player::resources::Resource;
use crate::game::game::Game;
use crate::actions::payment::Payment;

/// Deferred action: Select payment
/// Asks the player to select how to pay for something
pub struct SelectPaymentDeferred {
    player_id: PlayerId,
    amount: u32,
    // For now, simplified - will be enhanced when we have full payment options
}

impl SelectPaymentDeferred {
    /// Create a new SelectPayment deferred action
    pub fn new(player_id: PlayerId, amount: u32) -> Self {
        Self {
            player_id,
            amount,
        }
    }
}

impl DeferredAction for SelectPaymentDeferred {
    fn priority(&self) -> Priority {
        Priority::Cost
    }

    fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    fn execute(&mut self, game: &mut Game) -> Result<DeferredActionResult, String> {
        // For now, simplified implementation
        // In a full implementation, this would prompt the player for payment selection
        // For Phase 6, we'll just use M€ if available, otherwise return NeedsInput
        
        let player = game.get_player_mut(&self.player_id)
            .ok_or_else(|| format!("Player {} not found", self.player_id))?;

        if self.amount == 0 {
            return Ok(DeferredActionResult::Completed);
        }

        if player.resources.megacredits >= self.amount {
            // Auto-pay with M€ if available
            player.resources.subtract(Resource::Megacredits, self.amount);
            Ok(DeferredActionResult::Completed)
        } else {
            // Need player input for payment selection
            Ok(DeferredActionResult::NeedsInput)
        }
    }
}

/// Deferred action: Gain resources
/// Gives resources to a player
pub struct GainResourcesDeferred {
    player_id: PlayerId,
    resource: Resource,
    amount: u32,
}

impl GainResourcesDeferred {
    /// Create a new GainResources deferred action
    pub fn new(player_id: PlayerId, resource: Resource, amount: u32) -> Self {
        Self {
            player_id,
            resource,
            amount,
        }
    }
}

impl DeferredAction for GainResourcesDeferred {
    fn priority(&self) -> Priority {
        Priority::GainResourceOrProduction
    }

    fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    fn execute(&mut self, game: &mut Game) -> Result<DeferredActionResult, String> {
        let player = game.get_player_mut(&self.player_id)
            .ok_or_else(|| format!("Player {} not found", self.player_id))?;

        player.resources.add(self.resource, self.amount);
        Ok(DeferredActionResult::Completed)
    }
}

/// Deferred action: Place tile
/// Asks the player to place a tile on the board
pub struct PlaceTileDeferred {
    player_id: PlayerId,
    tile_type: String, // Simplified for Phase 6
}

impl PlaceTileDeferred {
    /// Create a new PlaceTile deferred action
    pub fn new(player_id: PlayerId, tile_type: String) -> Self {
        Self {
            player_id,
            tile_type,
        }
    }
}

impl DeferredAction for PlaceTileDeferred {
    fn priority(&self) -> Priority {
        Priority::Default
    }

    fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    fn execute(&mut self, _game: &mut Game) -> Result<DeferredActionResult, String> {
        // For Phase 6, simplified - tile placement will be fully implemented when board system is complete
        // For now, return NeedsInput to indicate player must choose a space
        Ok(DeferredActionResult::NeedsInput)
    }
}

/// Deferred action: Draw cards
/// Draws cards from the deck for a player
pub struct DrawCardsDeferred {
    player_id: PlayerId,
    count: u32,
}

impl DrawCardsDeferred {
    /// Create a new DrawCards deferred action
    pub fn new(player_id: PlayerId, count: u32) -> Self {
        Self {
            player_id,
            count,
        }
    }
}

impl DeferredAction for DrawCardsDeferred {
    fn priority(&self) -> Priority {
        Priority::DrawCards
    }

    fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    fn execute(&mut self, game: &mut Game) -> Result<DeferredActionResult, String> {
        // For Phase 6, simplified - card drawing will be fully implemented when deck system is complete
        // For now, we'll just add placeholder card IDs to the player's hand
        let player = game.get_player_mut(&self.player_id)
            .ok_or_else(|| format!("Player {} not found", self.player_id))?;

        // Placeholder: Add dummy card IDs
        for i in 0..self.count {
            player.add_card_to_hand(format!("drawn_card_{}", i));
        }

        Ok(DeferredActionResult::Completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardType;

    #[test]
    fn test_select_payment_deferred() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        // Give player enough M€
        game.players[0].resources.add(Resource::Megacredits, 10);
        
        let mut action = SelectPaymentDeferred::new("p1".to_string(), 5);
        let result = action.execute(&mut game).unwrap();
        assert_eq!(result, DeferredActionResult::Completed);
        assert_eq!(game.players[0].resources.megacredits, 5);
    }

    #[test]
    fn test_select_payment_deferred_insufficient() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        // Player has insufficient M€
        game.players[0].resources.add(Resource::Megacredits, 3);
        
        let mut action = SelectPaymentDeferred::new("p1".to_string(), 5);
        let result = action.execute(&mut game).unwrap();
        assert_eq!(result, DeferredActionResult::NeedsInput);
    }

    #[test]
    fn test_gain_resources_deferred() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        let mut action = GainResourcesDeferred::new("p1".to_string(), Resource::Steel, 5);
        let result = action.execute(&mut game).unwrap();
        assert_eq!(result, DeferredActionResult::Completed);
        assert_eq!(game.players[0].resources.steel, 5);
    }

    #[test]
    fn test_draw_cards_deferred() {
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        let mut action = DrawCardsDeferred::new("p1".to_string(), 3);
        let result = action.execute(&mut game).unwrap();
        assert_eq!(result, DeferredActionResult::Completed);
        assert_eq!(game.players[0].cards_in_hand.len(), 3);
    }
}

