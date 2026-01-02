
/// Payment method for flexible payment system
/// Supports multiple payment methods with resource conversion
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PaymentMethod {
    /// Pay with megacredits (standard currency)
    MegaCredits(u32),
    /// Pay with steel (converted to M€ at 1:2 ratio for building tags: 1 steel = 2 M€)
    Steel(u32),
    /// Pay with titanium (converted to M€ at 1:3 ratio for space tags: 1 titanium = 3 M€)
    Titanium(u32),
    /// Pay with heat (if Helion corporation ability active, 1:1 ratio)
    Heat(u32),
    /// Pay with plants (if Martian Lumber Corp ability active, for building tags)
    Plants(u32),
}

/// Payment struct for flexible payment system
/// Supports multiple payment methods and resource conversion
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Payment {
    /// Payment methods (can combine multiple methods)
    pub methods: Vec<PaymentMethod>,
    /// Minimum resources to keep (reserve units)
    /// Player must keep at least this amount of each resource
    pub reserve: PaymentReserve,
}

/// Reserve units (minimum resources to keep)
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentReserve {
    pub megacredits: u32,
    pub steel: u32,
    pub titanium: u32,
    pub heat: u32,
    pub plants: u32,
}

impl Payment {
    /// Create a new payment with megacredits only
    pub fn with_megacredits(amount: u32) -> Self {
        Self {
            methods: vec![PaymentMethod::MegaCredits(amount)],
            reserve: PaymentReserve::default(),
        }
    }

    /// Create a new payment with multiple methods
    pub fn new(methods: Vec<PaymentMethod>) -> Self {
        Self {
            methods,
            reserve: PaymentReserve::default(),
        }
    }

    /// Add reserve units
    pub fn with_reserve(mut self, reserve: PaymentReserve) -> Self {
        self.reserve = reserve;
        self
    }

    /// Calculate total cost in megacredits
    /// This converts all payment methods to M€ equivalent
    /// Note: Conversion ratios depend on card tags (building vs space)
    /// For now, we use default ratios (will be enhanced when we have card tags)
    pub fn total_cost_mc(&self, is_building_tag: bool, is_space_tag: bool) -> u32 {
        self.methods.iter().map(|method| {
            match method {
                PaymentMethod::MegaCredits(amount) => *amount,
                PaymentMethod::Steel(amount) => {
                    // Steel converts at 1:2 for building tags (1 steel = 2 M€), otherwise not usable
                    if is_building_tag {
                        *amount * 2
                    } else {
                        0 // Steel can only be used for building tags
                    }
                }
                PaymentMethod::Titanium(amount) => {
                    // Titanium converts at 1:3 for space tags (1 titanium = 3 M€), otherwise not usable
                    if is_space_tag {
                        *amount * 3
                    } else {
                        0 // Titanium can only be used for space tags
                    }
                }
                PaymentMethod::Heat(amount) => {
                    // Heat converts at 1:1 if Helion corporation ability active
                    // For now, we'll assume it's not active (will be enhanced later)
                    *amount
                }
                PaymentMethod::Plants(amount) => {
                    // Plants can be used for building tags if Martian Lumber Corp active
                    // "plants may be used as 3 M€ each" means 1 plant = 3 M€
                    if is_building_tag {
                        amount * 3 // 1 plant = 3 M€ for building tags (if Martian Lumber Corp active)
                    } else {
                        0
                    }
                }
            }
        }).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_with_megacredits() {
        let payment = Payment::with_megacredits(10);
        assert_eq!(payment.total_cost_mc(false, false), 10);
    }

    #[test]
    fn test_payment_steel_building_tag() {
        let payment = Payment::new(vec![PaymentMethod::Steel(4)]);
        // Steel: 4 steel = 8 M€ for building tags (1 steel = 2 M€)
        assert_eq!(payment.total_cost_mc(true, false), 8);
    }

    #[test]
    fn test_payment_steel_non_building_tag() {
        let payment = Payment::new(vec![PaymentMethod::Steel(4)]);
        // Steel can only be used for building tags
        assert_eq!(payment.total_cost_mc(false, false), 0);
    }

    #[test]
    fn test_payment_titanium_space_tag() {
        let payment = Payment::new(vec![PaymentMethod::Titanium(6)]);
        // Titanium: 6 titanium = 18 M€ for space tags (1 titanium = 3 M€)
        assert_eq!(payment.total_cost_mc(false, true), 18);
    }

    #[test]
    fn test_payment_titanium_non_space_tag() {
        let payment = Payment::new(vec![PaymentMethod::Titanium(6)]);
        // Titanium can only be used for space tags
        assert_eq!(payment.total_cost_mc(false, false), 0);
    }

    #[test]
    fn test_payment_multiple_methods() {
        let payment = Payment::new(vec![
            PaymentMethod::MegaCredits(5),
            PaymentMethod::Steel(4), // 8 M€ for building tags (1 steel = 2 M€)
        ]);
        assert_eq!(payment.total_cost_mc(true, false), 13);
    }

    #[test]
    fn test_payment_heat_conversion() {
        // Heat converts at 1:1 if Helion corporation ability active
        // For now, we assume it's not active, so heat is not usable
        // This test verifies the current behavior (will be enhanced when Helion is implemented)
        let payment = Payment::new(vec![PaymentMethod::Heat(8)]);
        // Currently heat is not usable for payment (will be 1:1 when Helion is active)
        assert_eq!(payment.total_cost_mc(false, false), 8);
    }

    #[test]
    fn test_payment_plants_conversion() {
        // Plants convert at 1:3 for building tags if Martian Lumber Corp active
        // "plants may be used as 3 M€ each" means 1 plant = 3 M€
        let payment = Payment::new(vec![PaymentMethod::Plants(3)]);
        // Plants: 3 plants = 9 M€ for building tags (1 plant = 3 M€)
        assert_eq!(payment.total_cost_mc(true, false), 9);
        
        // Plants cannot be used for non-building tags
        assert_eq!(payment.total_cost_mc(false, false), 0);
    }

    #[test]
    fn test_payment_reserve_units() {
        use crate::player::Player;
        use crate::player::resources::Resource;
        use crate::actions::action_executor::ActionExecutor;

        let player = Player::new("p1".to_string(), "Player 1".to_string());
        let mut player_with_resources = player;
        player_with_resources.resources.add(Resource::Megacredits, 20);
        player_with_resources.resources.add(Resource::Steel, 10);
        player_with_resources.resources.add(Resource::Titanium, 10);
        player_with_resources.resources.add(Resource::Heat, 10);
        player_with_resources.resources.add(Resource::Plants, 10);

        // Payment with reserve: must keep at least 5 M€, 3 steel, 2 titanium
        let mut payment = Payment::with_megacredits(12);
        payment.reserve.megacredits = 5;
        payment.reserve.steel = 3;
        payment.reserve.titanium = 2;

        // Should succeed: 20 M€ - 5 reserve = 15 available, need 12
        assert!(ActionExecutor::validate_payment(&payment, &player_with_resources, false, false).is_ok());

        // Should fail: 20 M€ - 5 reserve = 15 available, need 16
        let mut payment_too_much = Payment::with_megacredits(16);
        payment_too_much.reserve.megacredits = 5;
        assert!(ActionExecutor::validate_payment(&payment_too_much, &player_with_resources, false, false).is_err());

        // Test steel reserve
        let mut payment_steel = Payment::new(vec![PaymentMethod::Steel(6)]);
        payment_steel.reserve.steel = 3;
        // Should succeed: 10 steel - 3 reserve = 7 available, need 6
        assert!(ActionExecutor::validate_payment(&payment_steel, &player_with_resources, true, false).is_ok());

        // Should fail: 10 steel - 3 reserve = 7 available, need 8
        let mut payment_steel_too_much = Payment::new(vec![PaymentMethod::Steel(8)]);
        payment_steel_too_much.reserve.steel = 3;
        assert!(ActionExecutor::validate_payment(&payment_steel_too_much, &player_with_resources, true, false).is_err());
    }
}

