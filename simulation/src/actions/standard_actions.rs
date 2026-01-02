use crate::player::Player;

/// Standard actions (Convert Plants, Convert Heat)
pub struct StandardActions;

impl StandardActions {
    /// Validate if a player can convert plants to greenery
    /// Requires 8 plants
    pub fn can_convert_plants(player: &Player) -> Result<(), String> {
        let plants = player.resources.get(crate::player::resources::Resource::Plants);
        if plants < 8 {
            return Err(format!("Convert Plants requires 8 plants, but player has {plants}"));
        }
        Ok(())
    }

    /// Execute convert plants action
    /// Spend 8 plants to place 1 greenery tile (raises oxygen)
    pub fn convert_plants(player: &mut Player) -> Result<(), String> {
        Self::can_convert_plants(player)?;
        player.resources.subtract(
            crate::player::resources::Resource::Plants,
            8,
        );
        // Greenery placement and oxygen increase will be handled in action executor
        Ok(())
    }

    /// Validate if a player can convert heat to TR
    /// Requires 8 heat
    pub fn can_convert_heat(player: &Player) -> Result<(), String> {
        let heat = player.resources.get(crate::player::resources::Resource::Heat);
        if heat < 8 {
            return Err(format!("Convert Heat requires 8 heat, but player has {heat}"));
        }
        Ok(())
    }

    /// Execute convert heat action
    /// Spend 8 heat to raise TR by 1
    pub fn convert_heat(player: &mut Player) -> Result<(), String> {
        Self::can_convert_heat(player)?;
        player.resources.subtract(
            crate::player::resources::Resource::Heat,
            8,
        );
        player.terraform_rating += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_plants_validation() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        assert!(StandardActions::can_convert_plants(&player).is_err());

        player.resources.add(crate::player::resources::Resource::Plants, 8);
        assert!(StandardActions::can_convert_plants(&player).is_ok());
    }

    #[test]
    fn test_convert_plants_execution() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        player.resources.add(crate::player::resources::Resource::Plants, 10);

        assert!(StandardActions::convert_plants(&mut player).is_ok());
        assert_eq!(player.resources.get(crate::player::resources::Resource::Plants), 2);
    }

    #[test]
    fn test_convert_heat_validation() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        assert!(StandardActions::can_convert_heat(&player).is_err());

        player.resources.add(crate::player::resources::Resource::Heat, 8);
        assert!(StandardActions::can_convert_heat(&player).is_ok());
    }

    #[test]
    fn test_convert_heat_execution() {
        let mut player = Player::new("p1".to_string(), "Player 1".to_string());
        let initial_tr = player.terraform_rating;
        player.resources.add(crate::player::resources::Resource::Heat, 10);

        assert!(StandardActions::convert_heat(&mut player).is_ok());
        assert_eq!(player.resources.get(crate::player::resources::Resource::Heat), 2);
        assert_eq!(player.terraform_rating, initial_tr + 1);
    }
}

