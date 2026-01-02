/// Card ID type (simple identifier for Phase 4)
/// In Phase 5, this will be expanded to a full Card struct
pub type CardId = String;

/// Minimal card structure for Phase 4
/// This is just enough for testing standard projects like "Sell Patents"
/// In Phase 5, this will be expanded to a full Card struct with behavior, tags, etc.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MinimalCard {
    /// Card identifier
    pub id: CardId,
    /// Card name for logging
    pub name: String,
}

impl MinimalCard {
    /// Create a new minimal card
    pub fn new(id: CardId, name: String) -> Self {
        Self { id, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_card_creation() {
        let card = MinimalCard::new("card1".to_string(), "Test Card".to_string());
        assert_eq!(card.id, "card1");
        assert_eq!(card.name, "Test Card");
    }
}

