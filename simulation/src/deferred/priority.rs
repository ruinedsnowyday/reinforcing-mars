/// Priority levels for deferred actions
/// Lower values execute first (higher priority)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum Priority {
    /// Cost of a blue card action, or paying Reds costs. Must happen before the effects.
    Cost = 0,
    /// Draw cards
    DrawCards = 10,
    /// Place ocean tile
    PlaceOceanTile = 20,
    /// Default priority - anything that doesn't fit into another category
    Default = 50,
    /// Gain resource or production
    GainResourceOrProduction = 60,
    /// Lose resource or production
    LoseResourceOrProduction = 70,
    /// Discard cards
    DiscardCards = 80,
    /// Back of the line - lowest priority
    BackOfTheLine = 100,
}

impl Priority {
    /// Get the numeric value of the priority (lower = higher priority)
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Cost < Priority::Default);
        assert!(Priority::DrawCards < Priority::Default);
        assert!(Priority::Default < Priority::BackOfTheLine);
    }

    #[test]
    fn test_priority_value() {
        assert_eq!(Priority::Cost.value(), 0);
        assert_eq!(Priority::Default.value(), 50);
        assert_eq!(Priority::BackOfTheLine.value(), 100);
    }
}

