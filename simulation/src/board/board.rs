use crate::board::{Space, SpaceId, SpaceType, Tile};
use std::collections::HashMap;

/// Board type - only official boards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BoardType {
    /// Tharsis board (official)
    Tharsis,
    /// Hellas board (official)
    Hellas,
    /// Elysium board (official)
    Elysium,
}

/// Mars board representation
/// Supports the three official boards: Tharsis, Hellas, and Elysium
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Board {
    /// Board type
    board_type: BoardType,
    /// All spaces on the board (indexed by space ID)
    spaces: HashMap<SpaceId, Space>,
    /// Ocean spaces that have been placed (for tracking ocean count)
    placed_oceans: u32,
}

impl Board {
    /// Create a new board of the specified type
    /// For Phase 2, this creates a basic structure
    /// Full board initialization with all spaces will be added incrementally
    pub fn new(board_type: BoardType) -> Self {
        let mut board = Self {
            board_type,
            spaces: HashMap::new(),
            placed_oceans: 0,
        };

        // Initialize board with spaces based on type
        // For Phase 2, we'll create a minimal structure
        // Full space definitions will be added in later iterations
        board.initialize_spaces();
        board
    }

    /// Initialize spaces for the board
    /// This is a placeholder - full implementation will add all spaces with correct coordinates and bonuses
    fn initialize_spaces(&mut self) {
        // TODO: Implement full space initialization for each board type
        // For now, this is a placeholder that will be expanded
        match self.board_type {
            BoardType::Tharsis => {
                // Tharsis board has specific spaces - will be fully implemented
            }
            BoardType::Hellas => {
                // Hellas board has specific spaces - will be fully implemented
            }
            BoardType::Elysium => {
                // Elysium board has specific spaces - will be fully implemented
            }
        }
    }

    /// Get a space by ID
    pub fn get_space(&self, space_id: &SpaceId) -> Option<&Space> {
        self.spaces.get(space_id)
    }

    /// Get a mutable space by ID
    pub fn get_space_mut(&mut self, space_id: &SpaceId) -> Option<&mut Space> {
        self.spaces.get_mut(space_id)
    }

    /// Get all spaces
    pub fn all_spaces(&self) -> &HashMap<SpaceId, Space> {
        &self.spaces
    }

    /// Get all available spaces (no tile placed)
    pub fn available_spaces(&self) -> Vec<&Space> {
        self.spaces.values().filter(|s| s.is_available()).collect()
    }

    /// Get available spaces of a specific type
    pub fn available_spaces_of_type(&self, space_type: SpaceType) -> Vec<&Space> {
        self.spaces
            .values()
            .filter(|s| s.is_available() && s.space_type == space_type)
            .collect()
    }

    /// Get spaces that can accept a specific tile
    pub fn spaces_for_tile(&self, tile: &Tile) -> Vec<&Space> {
        self.spaces
            .values()
            .filter(|s| s.can_accept_tile(tile))
            .collect()
    }

    /// Place a tile on a space
    pub fn place_tile(
        &mut self,
        space_id: &SpaceId,
        tile: Tile,
        player_id: String,
    ) -> Result<(), String> {
        // Track ocean placement before moving tile
        let is_ocean = matches!(tile, Tile::Ocean);

        let space = self
            .spaces
            .get_mut(space_id)
            .ok_or_else(|| format!("Space {space_id} not found"))?;

        space.place_tile(tile, player_id)?;

        // Track ocean placement
        if is_ocean {
            self.placed_oceans += 1;
        }

        Ok(())
    }

    /// Get the number of placed ocean tiles
    pub fn placed_oceans(&self) -> u32 {
        self.placed_oceans
    }

    /// Get the board type
    pub fn board_type(&self) -> BoardType {
        self.board_type
    }

    /// Add a space to the board (for board initialization)
    pub fn add_space(&mut self, space: Space) {
        self.spaces.insert(space.id.clone(), space);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::SpaceBonus;

    #[test]
    fn test_board_creation() {
        let board = Board::new(BoardType::Tharsis);
        assert_eq!(board.board_type(), BoardType::Tharsis);
        assert_eq!(board.placed_oceans(), 0);
    }

    #[test]
    fn test_space_management() {
        let mut board = Board::new(BoardType::Tharsis);

        // Add a test space
        let space = Space::new(
            "test01".to_string(),
            0,
            0,
            SpaceType::Land,
            vec![SpaceBonus::Steel],
        );
        board.add_space(space);

        // Get the space
        assert!(board.get_space(&"test01".to_string()).is_some());
        assert_eq!(
            board.get_space(&"test01".to_string()).unwrap().space_type,
            SpaceType::Land
        );

        // Get available spaces
        let available = board.available_spaces();
        assert_eq!(available.len(), 1);

        // Place a tile
        assert!(board
            .place_tile(
                &"test01".to_string(),
                Tile::City,
                "player1".to_string()
            )
            .is_ok());

        // Space should no longer be available
        let available = board.available_spaces();
        assert_eq!(available.len(), 0);
    }

    #[test]
    fn test_ocean_tracking() {
        let mut board = Board::new(BoardType::Tharsis);

        // Add an ocean space
        let ocean_space = Space::new(
            "ocean01".to_string(),
            1,
            1,
            SpaceType::Ocean,
            vec![SpaceBonus::Ocean],
        );
        board.add_space(ocean_space);

        // Place an ocean tile
        assert!(board
            .place_tile(
                &"ocean01".to_string(),
                Tile::Ocean,
                "player1".to_string()
            )
            .is_ok());

        assert_eq!(board.placed_oceans(), 1);
    }

    #[test]
    fn test_spaces_for_tile() {
        let mut board = Board::new(BoardType::Tharsis);

        // Add land and ocean spaces
        let land_space = Space::new(
            "land01".to_string(),
            0,
            0,
            SpaceType::Land,
            vec![],
        );
        let ocean_space = Space::new(
            "ocean01".to_string(),
            1,
            1,
            SpaceType::Ocean,
            vec![],
        );
        board.add_space(land_space);
        board.add_space(ocean_space);

        // Find spaces for city (should be land)
        let city_spaces = board.spaces_for_tile(&Tile::City);
        assert_eq!(city_spaces.len(), 1);
        assert_eq!(city_spaces[0].space_type, SpaceType::Land);

        // Find spaces for ocean (should be ocean space)
        let ocean_spaces = board.spaces_for_tile(&Tile::Ocean);
        assert_eq!(ocean_spaces.len(), 1);
        assert_eq!(ocean_spaces[0].space_type, SpaceType::Ocean);
    }
}
