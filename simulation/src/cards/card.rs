use crate::cards::card_type::CardType;
use crate::cards::card_resource::CardResource;
use crate::cards::behavior::Behavior;
use crate::player::tags::Tag;

/// Card ID type (simple identifier)
pub type CardId = String;

/// Full Card struct for Phase 5
/// This expands the minimal card structure from Phase 4
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Card identifier (unique)
    pub id: CardId,
    /// Card name
    pub name: String,
    /// Card cost (in M€)
    pub cost: Option<u32>,
    /// Tags on this card
    pub tags: Vec<Tag>,
    /// Card type
    pub card_type: CardType,
    /// Behavior (optional, for Tier 1 cards)
    pub behavior: Option<Behavior>,
    /// Card resource type (if this card collects resources)
    pub resource_type: Option<CardResource>,
    /// Victory points (if any)
    pub victory_points: Option<i32>,
    /// Card requirements (simplified for Phase 5 - will be expanded later)
    /// For now, just store as a string description
    pub requirements: Option<String>,
}

impl Card {
    /// Create a new card
    pub fn new(
        id: CardId,
        name: String,
        card_type: CardType,
    ) -> Self {
        Self {
            id,
            name,
            cost: None,
            tags: Vec::new(),
            card_type,
            behavior: None,
            resource_type: None,
            victory_points: None,
            requirements: None,
        }
    }

    /// Set card cost
    pub fn with_cost(mut self, cost: u32) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Add tags to card
    pub fn with_tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = tags;
        self
    }

    /// Set card behavior
    pub fn with_behavior(mut self, behavior: Behavior) -> Self {
        self.behavior = Some(behavior);
        self
    }

    /// Set card resource type
    pub fn with_resource_type(mut self, resource_type: CardResource) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Set victory points
    pub fn with_victory_points(mut self, victory_points: i32) -> Self {
        self.victory_points = Some(victory_points);
        self
    }

    /// Set requirements
    pub fn with_requirements(mut self, requirements: String) -> Self {
        self.requirements = Some(requirements);
        self
    }

    /// Check if card has a specific tag
    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.contains(&tag)
    }

    /// Get card cost (defaults to 0 if not set)
    pub fn get_cost(&self) -> u32 {
        self.cost.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::behavior::Behavior;

    #[test]
    fn test_card_creation() {
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        );
        assert_eq!(card.id, "card1");
        assert_eq!(card.name, "Test Card");
        assert_eq!(card.card_type, CardType::Automated);
        assert_eq!(card.get_cost(), 0);
    }

    #[test]
    fn test_card_with_cost() {
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_cost(10);
        assert_eq!(card.get_cost(), 10);
    }

    #[test]
    fn test_card_with_tags() {
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_tags(vec![Tag::Building, Tag::Space]);
        assert!(card.has_tag(Tag::Building));
        assert!(card.has_tag(Tag::Space));
        assert!(!card.has_tag(Tag::Science));
    }

    #[test]
    fn test_card_with_behavior() {
        let behavior = Behavior::default();
        let card = Card::new(
            "card1".to_string(),
            "Test Card".to_string(),
            CardType::Automated,
        ).with_behavior(behavior.clone());
        assert_eq!(card.behavior, Some(behavior));
    }
}

