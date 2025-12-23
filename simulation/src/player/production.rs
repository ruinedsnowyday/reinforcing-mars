use crate::player::resources::Resource;

/// Tracks production values for each resource
/// Production cannot be negative except for megacredits (enforced via type system)
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Production {
    pub megacredits: i32,  // Can be negative
    pub steel: u32,
    pub titanium: u32,
    pub plants: u32,
    pub energy: u32,
    pub heat: u32,
}

impl Production {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, resource: Resource) -> i32 {
        match resource {
            Resource::Megacredits => self.megacredits,
            Resource::Steel => self.steel as i32,
            Resource::Titanium => self.titanium as i32,
            Resource::Plants => self.plants as i32,
            Resource::Energy => self.energy as i32,
            Resource::Heat => self.heat as i32,
        }
    }

    pub fn set(&mut self, resource: Resource, value: i32) {
        match resource {
            Resource::Megacredits => self.megacredits = value, // Can be negative
            Resource::Steel => self.steel = value.max(0) as u32,
            Resource::Titanium => self.titanium = value.max(0) as u32,
            Resource::Plants => self.plants = value.max(0) as u32,
            Resource::Energy => self.energy = value.max(0) as u32,
            Resource::Heat => self.heat = value.max(0) as u32,
        }
    }

    pub fn add(&mut self, resource: Resource, amount: i32) {
        match resource {
            Resource::Megacredits => self.megacredits += amount, // Can be negative
            Resource::Steel => {
                self.steel = (self.steel as i32 + amount).max(0) as u32;
            }
            Resource::Titanium => {
                self.titanium = (self.titanium as i32 + amount).max(0) as u32;
            }
            Resource::Plants => {
                self.plants = (self.plants as i32 + amount).max(0) as u32;
            }
            Resource::Energy => {
                self.energy = (self.energy as i32 + amount).max(0) as u32;
            }
            Resource::Heat => {
                self.heat = (self.heat as i32 + amount).max(0) as u32;
            }
        }
    }

    pub fn subtract(&mut self, resource: Resource, amount: i32) {
        self.add(resource, -amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_operations() {
        let mut production = Production::new();
        
        production.add(Resource::Megacredits, 5);
        assert_eq!(production.megacredits, 5);
        
        production.subtract(Resource::Megacredits, 2);
        assert_eq!(production.megacredits, 3);
        
        // Test that megacredits production can be negative
        production.subtract(Resource::Megacredits, 10);
        assert_eq!(production.megacredits, -7); // 3 - 10 = -7
        
        // Test that other production cannot be negative
        production.add(Resource::Steel, 5);
        production.subtract(Resource::Steel, 10);
        assert_eq!(production.steel, 0); // Clamped to 0
    }
}

