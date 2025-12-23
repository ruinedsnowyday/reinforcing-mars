use crate::player::resources::Resources;
use crate::player::tags::Tags;
use crate::player::production::Production;

/// Player ID type (simple wrapper around String)
pub type PlayerId = String;

/// Player struct - tracks player state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    
    /// Player resources
    pub resources: Resources,
    
    /// Production values
    pub production: Production,
    
    /// Terraform rating (starts at 20)
    pub terraform_rating: i32,
    
    /// Tags owned by this player
    pub tags: Tags,
    
    /// Cards in hand (minimal structure for Phase 4, expanded in Phase 5)
    /// For now, just store card IDs as strings
    pub cards_in_hand: Vec<String>,
    
    /// Played cards (minimal structure for Phase 4, expanded in Phase 5)
    /// For now, just store card IDs as strings
    pub played_cards: Vec<String>,
    
    /// Victory points breakdown (for tracking VP sources)
    pub victory_points: i32,
    
    /// Draft state: cards currently in draft hand
    pub draft_hand: Vec<String>,
    
    /// Draft state: cards drafted during current draft iteration
    pub drafted_cards: Vec<String>,
    
    /// Draft state: whether this player needs to make a draft selection
    pub needs_to_draft: bool,
    
    /// Research phase: corporation cards dealt to this player
    pub dealt_corporation_cards: Vec<String>,
    
    /// Research phase: selected corporation card
    pub selected_corporation: Option<String>,
    
    /// Research phase: selected prelude cards (2 cards)
    pub selected_preludes: Vec<String>,
    
    /// Research phase: prelude cards dealt to this player
    pub dealt_prelude_cards: Vec<String>,
}

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self {
            id,
            name,
            resources: Resources::new(),
            production: Production::new(),
            terraform_rating: 20, // Starting TR
            tags: Tags::new(),
            cards_in_hand: Vec::new(),
            played_cards: Vec::new(),
            victory_points: 0,
            draft_hand: Vec::new(),
            drafted_cards: Vec::new(),
            needs_to_draft: false,
            dealt_corporation_cards: Vec::new(),
            selected_corporation: None,
            selected_preludes: Vec::new(),
            dealt_prelude_cards: Vec::new(),
        }
    }

    /// Add a card to hand
    pub fn add_card_to_hand(&mut self, card_id: String) {
        self.cards_in_hand.push(card_id);
    }

    /// Remove a card from hand
    pub fn remove_card_from_hand(&mut self, card_id: &str) -> bool {
        if let Some(pos) = self.cards_in_hand.iter().position(|x| x == card_id) {
            self.cards_in_hand.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add a card to played cards
    pub fn add_played_card(&mut self, card_id: String) {
        self.played_cards.push(card_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("p1".to_string(), "Player 1".to_string());
        assert_eq!(player.terraform_rating, 20);
        assert_eq!(player.resources.megacredits, 0);
    }

    #[test]
    fn test_card_hand_management() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        
        player.add_card_to_hand("card1".to_string());
        assert_eq!(player.cards_in_hand.len(), 1);
        
        assert!(player.remove_card_from_hand("card1"));
        assert_eq!(player.cards_in_hand.len(), 0);
    }
}

