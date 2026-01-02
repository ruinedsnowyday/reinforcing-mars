use crate::player::resources::Resource;
use crate::game::global_params::GlobalParameter;
use crate::cards::card_resource::CardResource;

/// Behavior represents declarative card effects
/// This is used for Tier 1 cards (80% of cards) that can be defined declaratively
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Behavior {
    /// Gain or lose production
    pub production: Option<ProductionChange>,
    /// Gain or lose resources (stock)
    pub stock: Option<StockChange>,
    /// Gain standard resources (M€, steel, titanium, plants, energy, heat)
    pub standard_resource: Option<StandardResourceGain>,
    /// Add resources to this card itself
    pub add_resources: Option<CardResourceGain>,
    /// Gain or lose terraform rating
    pub tr: Option<i32>,
    /// Raise global parameters
    pub global: Option<GlobalParameterChange>,
    /// Place a city tile
    pub city: Option<TilePlacement>,
    /// Place a greenery tile (also raises oxygen)
    pub greenery: Option<TilePlacement>,
    /// Place an ocean tile
    pub ocean: Option<TilePlacement>,
    /// Place a custom tile
    pub tile: Option<CustomTilePlacement>,
    /// Draw cards from deck
    pub draw_cards: Option<u32>,
    /// Raise titanium value (for cards like Advanced Alloys)
    pub titanium_value: Option<i32>,
    /// Raise steel value (for cards like Advanced Alloys)
    pub steel_value: Option<i32>,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            production: None,
            stock: None,
            standard_resource: None,
            add_resources: None,
            tr: None,
            global: None,
            city: None,
            greenery: None,
            ocean: None,
            tile: None,
            draw_cards: None,
            titanium_value: None,
            steel_value: None,
        }
    }
}

/// Production change (can be positive or negative)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProductionChange {
    pub megacredits: Option<i32>,
    pub steel: Option<i32>,
    pub titanium: Option<i32>,
    pub plants: Option<i32>,
    pub energy: Option<i32>,
    pub heat: Option<i32>,
}

/// Stock change (can be positive or negative)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StockChange {
    pub megacredits: Option<i32>,
    pub steel: Option<i32>,
    pub titanium: Option<i32>,
    pub plants: Option<i32>,
    pub energy: Option<i32>,
    pub heat: Option<i32>,
}

/// Standard resource gain
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StandardResourceGain {
    pub resource: Resource,
    pub amount: u32,
}

/// Card resource gain (add resources to card)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CardResourceGain {
    pub resource: CardResource,
    pub amount: u32,
}

/// Global parameter change
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GlobalParameterChange {
    pub parameter: GlobalParameter,
    pub steps: i32, // Can be positive (increase) or negative (decrease)
}

/// Tile placement (simplified for Phase 5)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TilePlacement {
    /// Optional space ID (if None, player chooses)
    pub space_id: Option<String>,
}

/// Custom tile placement
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomTilePlacement {
    pub tile_type: String, // Simplified for Phase 5
    pub space_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavior_default() {
        let behavior = Behavior::default();
        assert_eq!(behavior.production, None);
        assert_eq!(behavior.tr, None);
    }

    #[test]
    fn test_behavior_production() {
        let mut behavior = Behavior::default();
        behavior.production = Some(ProductionChange {
            megacredits: Some(1),
            steel: Some(1),
            ..Default::default()
        });
        assert!(behavior.production.is_some());
    }
}

impl Default for ProductionChange {
    fn default() -> Self {
        Self {
            megacredits: None,
            steel: None,
            titanium: None,
            plants: None,
            energy: None,
            heat: None,
        }
    }
}

impl Default for StockChange {
    fn default() -> Self {
        Self {
            megacredits: None,
            steel: None,
            titanium: None,
            plants: None,
            energy: None,
            heat: None,
        }
    }
}

