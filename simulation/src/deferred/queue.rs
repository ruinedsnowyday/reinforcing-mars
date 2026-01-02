use std::collections::VecDeque;
use crate::deferred::deferred_action::{DeferredAction, DeferredActionResult};
use crate::deferred::priority::Priority;
use crate::game::game::Game;

/// Entry in the deferred action queue
struct QueueEntry {
    action: Box<dyn DeferredAction>,
    insertion_order: u64,
}

impl QueueEntry {
    fn new(action: Box<dyn DeferredAction>, insertion_order: u64) -> Self {
        Self {
            action,
            insertion_order,
        }
    }

    /// Compare two queue entries for ordering
    /// Lower priority value = higher priority = execute first
    /// For same priority, earlier insertion = execute first
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let priority_cmp = self.action.priority().value().cmp(&other.action.priority().value());
        if priority_cmp != std::cmp::Ordering::Equal {
            return priority_cmp;
        }
        // Same priority: earlier insertion order = higher priority
        self.insertion_order.cmp(&other.insertion_order)
    }
}

/// Priority queue for deferred actions
/// Actions are executed in priority order (lower priority value = higher priority = execute first)
pub struct DeferredActionQueue {
    queue: VecDeque<QueueEntry>,
    insertion_counter: u64,
}

impl DeferredActionQueue {
    /// Create a new empty deferred action queue
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            insertion_counter: 0,
        }
    }

    /// Push a deferred action onto the queue
    /// The action will be inserted in priority order
    pub fn push(&mut self, action: Box<dyn DeferredAction>) {
        let insertion_order = self.insertion_counter;
        self.insertion_counter += 1;

        let entry = QueueEntry::new(action, insertion_order);
        
        // Insert in sorted order (by priority, then insertion order)
        let pos = self.queue.binary_search_by(|e| e.cmp(&entry))
            .unwrap_or_else(|pos| pos);
        self.queue.insert(pos, entry);
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get the number of actions in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Execute the next deferred action in the queue
    /// Returns:
    /// - Some(Ok(DeferredActionResult)) if an action was executed
    /// - None if the queue is empty
    /// - Some(Err(String)) if the action execution failed
    pub fn execute_next(&mut self, game: &mut Game) -> Option<Result<DeferredActionResult, String>> {
        if self.queue.is_empty() {
            return None;
        }

        let mut entry = self.queue.pop_front().unwrap();
        let result = entry.action.execute(game);

        // If the action needs input, put it back at the front (same priority)
        if let Ok(DeferredActionResult::NeedsInput) = result {
            // Re-insert at the front to maintain priority order
            self.queue.push_front(entry);
        } else if let Ok(DeferredActionResult::Remove) = result {
            // Action requested removal, don't re-insert
        } else if result.is_err() {
            // Action failed, don't re-insert
        }
        // If Completed, action is removed (already popped)

        Some(result)
    }

    /// Execute all deferred actions in the queue until it's empty or an action needs input
    /// Returns the number of actions executed
    pub fn execute_all(&mut self, game: &mut Game) -> usize {
        let mut executed = 0;
        while !self.queue.is_empty() {
            match self.execute_next(game) {
                Some(Ok(DeferredActionResult::Completed)) => {
                    executed += 1;
                }
                Some(Ok(DeferredActionResult::NeedsInput)) => {
                    // Stop execution, action needs player input
                    break;
                }
                Some(Ok(DeferredActionResult::Remove)) => {
                    executed += 1;
                }
                Some(Err(_)) => {
                    // Action failed, continue with next
                    executed += 1;
                }
                None => {
                    // Queue is empty
                    break;
                }
            }
        }
        executed
    }

    /// Get the next action's priority (if any)
    pub fn next_priority(&self) -> Option<Priority> {
        self.queue.front().map(|e| e.action.priority())
    }
}

impl Default for DeferredActionQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deferred::deferred_action::SimpleDeferredAction;
    use crate::board::BoardType;

    #[test]
    fn test_queue_new() {
        let queue = DeferredActionQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_queue_push() {
        let mut queue = DeferredActionQueue::new();
        let action = Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Default,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        ));
        queue.push(action);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_priority_ordering() {
        let mut queue = DeferredActionQueue::new();
        
        // Push actions in reverse priority order
        queue.push(Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::BackOfTheLine,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        )));
        queue.push(Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Default,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        )));
        queue.push(Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Cost,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        )));

        // Cost should execute first (lowest priority value)
        assert_eq!(queue.next_priority(), Some(Priority::Cost));
        
        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Execute and verify order
        let result1 = queue.execute_next(&mut game).unwrap().unwrap();
        assert_eq!(result1, DeferredActionResult::Completed);
        assert_eq!(queue.next_priority(), Some(Priority::Default));

        let result2 = queue.execute_next(&mut game).unwrap().unwrap();
        assert_eq!(result2, DeferredActionResult::Completed);
        assert_eq!(queue.next_priority(), Some(Priority::BackOfTheLine));

        let result3 = queue.execute_next(&mut game).unwrap().unwrap();
        assert_eq!(result3, DeferredActionResult::Completed);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_execute_all() {
        let mut queue = DeferredActionQueue::new();
        
        for _i in 0..5 {
            queue.push(Box::new(SimpleDeferredAction::new(
                "p1".to_string(),
                Priority::Default,
                move |_game, _player_id| {
                    // Verify execution order
                    Ok(DeferredActionResult::Completed)
                },
            )));
        }

        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        let executed = queue.execute_all(&mut game);
        assert_eq!(executed, 5);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_needs_input() {
        let mut queue = DeferredActionQueue::new();
        
        queue.push(Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Default,
            |_game, _player_id| Ok(DeferredActionResult::NeedsInput),
        )));
        queue.push(Box::new(SimpleDeferredAction::new(
            "p1".to_string(),
            Priority::Default,
            |_game, _player_id| Ok(DeferredActionResult::Completed),
        )));

        let mut game = Game::new(
            "test".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Execute all should stop at NeedsInput
        let executed = queue.execute_all(&mut game);
        assert_eq!(executed, 0); // First action needs input, so none executed
        assert_eq!(queue.len(), 2); // Both still in queue

        // Execute next should return NeedsInput and keep action in queue
        let result = queue.execute_next(&mut game).unwrap().unwrap();
        assert_eq!(result, DeferredActionResult::NeedsInput);
        assert_eq!(queue.len(), 2); // Still in queue
    }
}

