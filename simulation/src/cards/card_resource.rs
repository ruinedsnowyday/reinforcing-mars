/// Card resources - resources that can be placed on cards
/// 
/// Note: Exclude unofficial card resources (Moon, Pathfinders, Star Wars, Underworld, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardResource {
    /// Animal resources (base game)
    Animal,
    /// Microbe resources (base game)
    Microbe,
    /// Fighter resources (base game)
    Fighter,
    /// Science resources (base game)
    Science,
    /// Floater resources (Venus Next expansion)
    Floater,
    /// Asteroid resources (Venus Next expansion)
    Asteroid,
    /// Camp resources (Colonies expansion)
    Camp,
    /// Preservation resources (Turmoil expansion)
    Preservation,
    /// Director resources (Prelude 2 expansion)
    Director,
    /// Disease resources (Promos)
    Disease,
    /// Graphene resources (Promos)
    Graphene,
    /// Hydroelectric resources (Promos)
    HydroelectricResource,
}

impl CardResource {
    /// Get all official card resources
    pub fn all() -> Vec<CardResource> {
        vec![
            CardResource::Animal,
            CardResource::Microbe,
            CardResource::Fighter,
            CardResource::Science,
            CardResource::Floater,
            CardResource::Asteroid,
            CardResource::Camp,
            CardResource::Preservation,
            CardResource::Director,
            CardResource::Disease,
            CardResource::Graphene,
            CardResource::HydroelectricResource,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_resource_all() {
        let all = CardResource::all();
        assert_eq!(all.len(), 12);
    }
}

