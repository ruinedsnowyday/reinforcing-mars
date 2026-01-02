use crate::deferred::priority::Priority;
use crate::player::PlayerId;
use crate::game::game::Game;

/// Trait for deferred actions
/// Deferred actions are queued operations that execute before normal player actions
pub trait DeferredAction: Send + Sync {
    /// Get the priority of this action (lower = higher priority)
    fn priority(&self) -> Priority;

    /// Get the player ID this action belongs to
    fn player_id(&self) -> &PlayerId;

    /// Execute the deferred action
    /// Returns Ok(()) if the action completed successfully
    /// Returns Err(String) if the action failed
    /// Returns Ok(()) if the action needs more input (will be handled by game flow)
    fn execute(&mut self, game: &mut Game) -> Result<DeferredActionResult, String>;
}

/// Result of executing a deferred action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferredActionResult {
    /// Action completed successfully
    Completed,
    /// Action needs player input (e.g., select payment, choose card)
    /// The action will remain in the queue until input is provided
    NeedsInput,
    /// Action should be removed from queue (e.g., skipped or cancelled)
    Remove,
}

/// Simple deferred action that executes a closure
pub struct SimpleDeferredAction {
    priority: Priority,
    player_id: PlayerId,
    execute_fn: Box<dyn FnMut(&mut Game, &PlayerId) -> Result<DeferredActionResult, String> + Send + Sync>,
}

impl SimpleDeferredAction {
    /// Create a new simple deferred action
    pub fn new<F>(
        player_id: PlayerId,
        priority: Priority,
        execute_fn: F,
    ) -> Self
    where
        F: FnMut(&mut Game, &PlayerId) -> Result<DeferredActionResult, String> + Send + Sync + 'static,
    {
        Self {
            priority,
            player_id,
            execute_fn: Box::new(execute_fn),
        }
    }
}

impl DeferredAction for SimpleDeferredAction {
    fn priority(&self) -> Priority {
        self.priority
    }

    fn player_id(&self) -> &PlayerId {
        &self.player_id
    }

    fn execute(&mut self, game: &mut Game) -> Result<DeferredActionResult, String> {
        (self.execute_fn)(game, &self.player_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BoardType;

    #[test]
    fn test_simple_deferred_action() {
        let mut action = SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Default,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        );

        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        assert_eq!(action.priority(), Priority::Default);
        assert_eq!(action.player_id(), "p1");
        let result = action.execute(&mut game).unwrap();
        assert_eq!(result, DeferredActionResult::Completed);
    }
}

