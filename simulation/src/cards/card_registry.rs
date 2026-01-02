use std::collections::HashMap;
use crate::cards::Card;
use crate::cards::CardId;

/// CardRegistry stores card definitions
/// Supports lookup by card ID
pub struct CardRegistry {
    cards: HashMap<CardId, Card>,
}

impl CardRegistry {
    /// Create a new empty card registry
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }

    /// Register a card in the registry
    pub fn register(&mut self, card: Card) {
        self.cards.insert(card.id.clone(), card);
    }

    /// Get a card by ID
    pub fn get(&self, card_id: &CardId) -> Option<&Card> {
        self.cards.get(card_id)
    }

    /// Get a mutable card by ID
    pub fn get_mut(&mut self, card_id: &CardId) -> Option<&mut Card> {
        self.cards.get_mut(card_id)
    }

    /// Check if a card exists in the registry
    pub fn contains(&self, card_id: &CardId) -> bool {
        self.cards.contains_key(card_id)
    }

    /// Get all card IDs in the registry
    pub fn all_card_ids(&self) -> Vec<CardId> {
        self.cards.keys().cloned().collect()
    }

    /// Get all cards in the registry
    pub fn all_cards(&self) -> Vec<&Card> {
        self.cards.values().collect()
    }

    /// Get the number of cards in the registry
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for CardRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::CardType;

    #[test]
    fn test_card_registry_new() {
        let registry = CardRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_card_registry_register() {
        let mut registry = CardRegistry::new();
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        );
        registry.register(card);
        assert_eq!(registry.len(), 1);
        assert!(registry.contains(&"card1".to_string()));
    }

    #[test]
    fn test_card_registry_get() {
        let mut registry = CardRegistry::new();
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        );
        registry.register(card);
        let retrieved = registry.get(&"card1".to_string());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Card");
    }

    #[test]
    fn test_card_registry_get_nonexistent() {
        let registry = CardRegistry::new();
        let retrieved = registry.get(&"nonexistent".to_string());
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_card_registry_all_card_ids() {
        let mut registry = CardRegistry::new();
        registry.register(Card::new("card1".to_string(), "Card 1".to_string(), CardType::Automated));
        registry.register(Card::new("card2".to_string(), "Card 2".to_string(), CardType::Active));
        let ids = registry.all_card_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"card1".to_string()));
        assert!(ids.contains(&"card2".to_string()));
    }
}

