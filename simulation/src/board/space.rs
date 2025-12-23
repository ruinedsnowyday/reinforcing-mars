use crate::board::tile::Tile;

/// Space type on the board
/// Only official space types are included
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpaceType {
    /// Land space - can place cities and greenery
    Land,
    /// Ocean space - can place ocean tiles
    Ocean,
    /// Colony space - for Colonies expansion
    Colony,
}

/// Space bonuses that can be granted when placing tiles
/// Only official bonuses are included
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpaceBonus {
    /// Titanium bonus
    Titanium,
    /// Steel bonus
    Steel,
    /// Plant bonus
    Plant,
    /// Draw card bonus
    DrawCard,
    /// Heat bonus
    Heat,
    /// Ocean bonus (for ocean tiles)
    Ocean,
}

/// A space on the Mars board
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Space {
    /// Unique space ID
    pub id: SpaceId,
    /// X coordinate on the board (or -1 for non-board spaces like colonies)
    pub x: i32,
    /// Y coordinate on the board (or -1 for non-board spaces like colonies)
    pub y: i32,
    /// Type of space
    pub space_type: SpaceType,
    /// Tile placed on this space (if any)
    pub tile: Option<Tile>,
    /// Player ID who owns this tile (if any)
    pub player_id: Option<String>,
    /// Bonuses granted when placing a tile on this space
    pub bonus: Vec<SpaceBonus>,
}

/// Space ID type
pub type SpaceId = String;

impl Space {
    /// Create a new space
    pub fn new(id: SpaceId, x: i32, y: i32, space_type: SpaceType, bonus: Vec<SpaceBonus>) -> Self {
        Self {
            id,
            x,
            y,
            space_type,
            tile: None,
            player_id: None,
            bonus,
        }
    }

    /// Check if this space is available (no tile placed)
    pub fn is_available(&self) -> bool {
        self.tile.is_none()
    }

    /// Check if this space can accept a specific tile type
    pub fn can_accept_tile(&self, tile: &Tile) -> bool {
        if !self.is_available() {
            return false;
        }

        match (self.space_type, tile) {
            (SpaceType::Ocean, Tile::Ocean) => true,
            (SpaceType::Land, Tile::City) => true,
            (SpaceType::Land, Tile::Greenery) => true,
            (SpaceType::Land, Tile::Special(_)) => true,
            (SpaceType::Colony, _) => false, // Colonies handled separately
            _ => false,
        }
    }

    /// Place a tile on this space
    pub fn place_tile(&mut self, tile: Tile, player_id: String) -> Result<(), String> {
        if !self.is_available() {
            return Err(format!("Space {} is already occupied", self.id));
        }

        if !self.can_accept_tile(&tile) {
            return Err(format!("Space {} cannot accept tile {tile:?}", self.id));
        }

        self.tile = Some(tile);
        self.player_id = Some(player_id);
        Ok(())
    }

    /// Remove tile from this space (for testing/debugging)
    pub fn remove_tile(&mut self) {
        self.tile = None;
        self.player_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_creation() {
        let space = Space::new(
            "01".to_string(),
            0,
            0,
            SpaceType::Land,
            vec![SpaceBonus::Steel, SpaceBonus::Plant],
        );

        assert_eq!(space.id, "01");
        assert_eq!(space.x, 0);
        assert_eq!(space.y, 0);
        assert_eq!(space.space_type, SpaceType::Land);
        assert!(space.is_available());
        assert_eq!(space.bonus.len(), 2);
    }

    #[test]
    fn test_tile_placement() {
        let mut space = Space::new(
            "01".to_string(),
            0,
            0,
            SpaceType::Land,
            vec![],
        );

        // Place a city
        assert!(space.place_tile(Tile::City, "player1".to_string()).is_ok());
        assert!(!space.is_available());
        assert_eq!(space.player_id, Some("player1".to_string()));

        // Try to place another tile (should fail)
        assert!(space.place_tile(Tile::Greenery, "player2".to_string()).is_err());
    }

    #[test]
    fn test_ocean_space() {
        let mut ocean_space = Space::new(
            "ocean01".to_string(),
            1,
            1,
            SpaceType::Ocean,
            vec![SpaceBonus::Ocean],
        );

        // Can place ocean
        assert!(ocean_space.can_accept_tile(&Tile::Ocean));
        assert!(ocean_space.place_tile(Tile::Ocean, "player1".to_string()).is_ok());

        // Cannot place city on ocean
        ocean_space.remove_tile();
        assert!(!ocean_space.can_accept_tile(&Tile::City));
    }
}
