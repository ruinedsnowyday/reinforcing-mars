/// Standard resources in Terraforming Mars
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Resource {
    /// MegaCredits - the standard currency
    Megacredits,
    /// Steel - used for building tags (1:2 conversion to M€: 1 steel = 2 M€)
    Steel,
    /// Titanium - used for space tags (1:3 conversion to M€: 1 titanium = 3 M€)
    Titanium,
    /// Plants - used for greenery tiles
    Plants,
    /// Energy - production resource
    Energy,
    /// Heat - production resource, can be converted to TR
    Heat,
}

impl Resource {
    /// Get all standard resources
    pub fn all() -> Vec<Resource> {
        vec![
            Resource::Megacredits,
            Resource::Steel,
            Resource::Titanium,
            Resource::Plants,
            Resource::Energy,
            Resource::Heat,
        ]
    }
}

/// Represents a collection of resources (quantities)
/// Resource quantities can NEVER be negative, including megacredits.
/// Note: Only megacredits PRODUCTION can be negative, not the resource quantity itself.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Resources {
    pub megacredits: u32,  // Resource quantity - cannot be negative
    pub steel: u32,
    pub titanium: u32,
    pub plants: u32,
    pub energy: u32,
    pub heat: u32,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, resource: Resource) -> u32 {
        match resource {
            Resource::Megacredits => self.megacredits,
            Resource::Steel => self.steel,
            Resource::Titanium => self.titanium,
            Resource::Plants => self.plants,
            Resource::Energy => self.energy,
            Resource::Heat => self.heat,
        }
    }

    pub fn set(&mut self, resource: Resource, value: u32) {
        match resource {
            Resource::Megacredits => self.megacredits = value,
            Resource::Steel => self.steel = value,
            Resource::Titanium => self.titanium = value,
            Resource::Plants => self.plants = value,
            Resource::Energy => self.energy = value,
            Resource::Heat => self.heat = value,
        }
    }

    pub fn add(&mut self, resource: Resource, amount: u32) {
        match resource {
            Resource::Megacredits => self.megacredits += amount,
            Resource::Steel => self.steel += amount,
            Resource::Titanium => self.titanium += amount,
            Resource::Plants => self.plants += amount,
            Resource::Energy => self.energy += amount,
            Resource::Heat => self.heat += amount,
        }
    }

    pub fn subtract(&mut self, resource: Resource, amount: u32) {
        match resource {
            Resource::Megacredits => {
                self.megacredits = self.megacredits.saturating_sub(amount);
            }
            Resource::Steel => {
                self.steel = self.steel.saturating_sub(amount);
            }
            Resource::Titanium => {
                self.titanium = self.titanium.saturating_sub(amount);
            }
            Resource::Plants => {
                self.plants = self.plants.saturating_sub(amount);
            }
            Resource::Energy => {
                self.energy = self.energy.saturating_sub(amount);
            }
            Resource::Heat => {
                self.heat = self.heat.saturating_sub(amount);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_operations() {
        let mut resources = Resources::new();
        
        // Test addition
        resources.add(Resource::Megacredits, 10);
        assert_eq!(resources.megacredits, 10);
        
        // Test subtraction
        resources.subtract(Resource::Megacredits, 3);
        assert_eq!(resources.megacredits, 7);
        
        // Test set
        resources.set(Resource::Steel, 5);
        assert_eq!(resources.steel, 5);
        
        // Test get
        assert_eq!(resources.get(Resource::Steel), 5);
        
        // Test that resources can't go negative (all resources, including megacredits)
        resources.subtract(Resource::Steel, 10);
        assert_eq!(resources.steel, 0); // Saturating subtract, not negative
        
        resources.subtract(Resource::Megacredits, 10);
        assert_eq!(resources.megacredits, 0); // Megacredits also can't go negative
    }
}

