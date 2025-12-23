/// Tile types that can be placed on the board
/// Simplified for Phase 2 - will be expanded in later phases
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Tile {
    /// City tile
    City,
    /// Greenery tile
    Greenery,
    /// Ocean tile
    Ocean,
    /// Special tile (e.g., from cards)
    /// The string represents the card name or special tile identifier
    Special(String),
}

impl Tile {
    /// Get all basic tile types (excluding Special)
    pub fn basic_types() -> Vec<Tile> {
        vec![Tile::City, Tile::Greenery, Tile::Ocean]
    }

    /// Check if this is a special tile
    pub fn is_special(&self) -> bool {
        matches!(self, Tile::Special(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_basic_types() {
        let basic = Tile::basic_types();
        assert_eq!(basic.len(), 3);
        assert!(basic.contains(&Tile::City));
        assert!(basic.contains(&Tile::Greenery));
        assert!(basic.contains(&Tile::Ocean));
    }

    #[test]
    fn test_special_tile() {
        let special = Tile::Special("Mining Area".to_string());
        assert!(special.is_special());
        assert!(!Tile::City.is_special());
    }
}
