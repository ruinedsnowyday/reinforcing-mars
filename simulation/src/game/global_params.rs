/// Global parameters in Terraforming Mars
/// 
/// Note: Moon tracks (MoonHabitatRate, MoonMiningRate, MoonLogisticsRate) 
/// are from the unofficial Moon expansion and should NOT be included.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GlobalParameter {
    /// Number of ocean tiles placed (0-9)
    Oceans,
    /// Oxygen level (0-14%)
    Oxygen,
    /// Temperature (-30 to +8 degrees C)
    Temperature,
    /// Venus scale level (0-30) - from Venus Next expansion
    Venus,
}

impl GlobalParameter {
    /// Get all official global parameters
    pub fn all() -> Vec<GlobalParameter> {
        vec![
            GlobalParameter::Oceans,
            GlobalParameter::Oxygen,
            GlobalParameter::Temperature,
            GlobalParameter::Venus,
        ]
    }
}

/// Maximum values for global parameters (user-facing)
pub const MAX_OCEANS: u32 = 9;
pub const MAX_OXYGEN: u32 = 14;
pub const MAX_TEMPERATURE: i32 = 8;
pub const MIN_TEMPERATURE: i32 = -30;
pub const MAX_VENUS: u32 = 30;

/// Step sizes for global parameters
pub const TEMPERATURE_STEP: i32 = 2;
pub const OXYGEN_STEP: u32 = 1;
pub const OCEANS_STEP: u32 = 1;
pub const VENUS_STEP: u32 = 2;

/// Maximum scale levels (internal representation: 0 to max_level - 1)
/// Temperature: 20 levels (0-19) representing -30, -28, ..., 0, 2, 4, 6, 8
/// Oxygen: 15 levels (0-14) representing 0, 1, 2, ..., 14
/// Oceans: 10 levels (0-9) representing 0, 1, 2, ..., 9
/// Venus: 16 levels (0-15) representing 0, 2, 4, ..., 28, 30
pub const TEMPERATURE_MAX_LEVEL: u8 = 20;
pub const OXYGEN_MAX_LEVEL: u8 = 15;
pub const OCEANS_MAX_LEVEL: u8 = 10;
pub const VENUS_MAX_LEVEL: u8 = 16;

/// Tracks global parameter values using internal scale representation
/// Internally stored as u8 scale values (0 to max_level - 1)
/// Use conversion methods to get/set user-facing values
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GlobalParameters {
    /// Internal scale: 0 to 9 (represents 0 to 9 oceans)
    oceans: u8,
    /// Internal scale: 0 to 14 (represents 0 to 14 oxygen)
    oxygen: u8,
    /// Internal scale: 0 to 19 (represents -30 to +8 temperature in steps of 2)
    temperature: u8,
    /// Internal scale: 0 to 15 (represents 0 to 30 venus in steps of 2)
    venus: u8,
}

impl GlobalParameters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the user-facing value for a parameter
    pub fn get(&self, param: GlobalParameter) -> i32 {
        match param {
            GlobalParameter::Oceans => Self::oceans_scale_to_value(self.oceans),
            GlobalParameter::Oxygen => Self::oxygen_scale_to_value(self.oxygen),
            GlobalParameter::Temperature => Self::temperature_scale_to_value(self.temperature),
            GlobalParameter::Venus => Self::venus_scale_to_value(self.venus),
        }
    }

    /// Convert internal scale (0-9) to user-facing oceans value (0-9)
    fn oceans_scale_to_value(scale: u8) -> i32 {
        scale as i32
    }

    /// Convert user-facing oceans value (0-9) to internal scale (0-9)
    fn oceans_value_to_scale(value: i32) -> u8 {
        value.max(0).min(MAX_OCEANS as i32) as u8
    }

    /// Convert internal scale (0-14) to user-facing oxygen value (0-14)
    fn oxygen_scale_to_value(scale: u8) -> i32 {
        scale as i32
    }

    /// Convert user-facing oxygen value (0-14) to internal scale (0-14)
    fn oxygen_value_to_scale(value: i32) -> u8 {
        value.max(0).min(MAX_OXYGEN as i32) as u8
    }

    /// Convert internal scale (0-19) to user-facing temperature value (-30 to +8 in steps of 2)
    fn temperature_scale_to_value(scale: u8) -> i32 {
        MIN_TEMPERATURE + (scale as i32 * TEMPERATURE_STEP)
    }

    /// Convert user-facing temperature value (-30 to +8) to internal scale (0-19)
    /// Rounds to nearest valid step
    fn temperature_value_to_scale(value: i32) -> u8 {
        let clamped = value.clamp(MIN_TEMPERATURE, MAX_TEMPERATURE);
        // Round to nearest step: (value - min) / step, rounded
        let steps_from_min = ((clamped - MIN_TEMPERATURE) + TEMPERATURE_STEP / 2) / TEMPERATURE_STEP;
        steps_from_min.max(0).min((TEMPERATURE_MAX_LEVEL - 1) as i32) as u8
    }

    /// Convert internal scale (0-15) to user-facing venus value (0-30 in steps of 2)
    fn venus_scale_to_value(scale: u8) -> i32 {
        (scale as u32 * VENUS_STEP) as i32
    }

    /// Convert user-facing venus value (0-30) to internal scale (0-15)
    /// Rounds to nearest valid step
    fn venus_value_to_scale(value: i32) -> u8 {
        let clamped = value.max(0).min(MAX_VENUS as i32) as u32;
        // Round to nearest step: (value + step/2) / step
        let scale = ((clamped + VENUS_STEP / 2) / VENUS_STEP) as u8;
        scale.min(VENUS_MAX_LEVEL - 1)
    }

    /// Set a global parameter to a specific value
    /// The value will be rounded to the nearest valid step boundary
    /// and clamped to min/max limits
    pub fn set(&mut self, param: GlobalParameter, value: i32) {
        match param {
            GlobalParameter::Oceans => {
                self.oceans = Self::oceans_value_to_scale(value);
            }
            GlobalParameter::Oxygen => {
                self.oxygen = Self::oxygen_value_to_scale(value);
            }
            GlobalParameter::Temperature => {
                self.temperature = Self::temperature_value_to_scale(value);
            }
            GlobalParameter::Venus => {
                self.venus = Self::venus_value_to_scale(value);
            }
        }
    }

    /// Increase a global parameter by a number of steps
    /// Only positive steps are allowed (global parameters can only increase)
    /// Returns the number of steps actually increased (may be less than requested if at maximum)
    pub fn increase(&mut self, param: GlobalParameter, steps: u32) -> u32 {
        if steps == 0 {
            return 0;
        }

        match param {
            GlobalParameter::Oceans => {
                let max_scale = (OCEANS_MAX_LEVEL - 1) as u32;
                let current_scale = self.oceans as u32;
                let available = max_scale.saturating_sub(current_scale);
                let actual_steps = steps.min(available);
                self.oceans = (current_scale + actual_steps) as u8;
                actual_steps
            }
            GlobalParameter::Oxygen => {
                let max_scale = (OXYGEN_MAX_LEVEL - 1) as u32;
                let current_scale = self.oxygen as u32;
                let available = max_scale.saturating_sub(current_scale);
                let actual_steps = steps.min(available);
                self.oxygen = (current_scale + actual_steps) as u8;
                actual_steps
            }
            GlobalParameter::Temperature => {
                let max_scale = (TEMPERATURE_MAX_LEVEL - 1) as u32;
                let current_scale = self.temperature as u32;
                let available = max_scale.saturating_sub(current_scale);
                let actual_steps = steps.min(available);
                self.temperature = (current_scale + actual_steps) as u8;
                actual_steps
            }
            GlobalParameter::Venus => {
                let max_scale = (VENUS_MAX_LEVEL - 1) as u32;
                let current_scale = self.venus as u32;
                let available = max_scale.saturating_sub(current_scale);
                let actual_steps = steps.min(available);
                self.venus = (current_scale + actual_steps) as u8;
                actual_steps
            }
        }
    }

    /// Check if a global parameter can be increased
    pub fn can_increase(&self, param: GlobalParameter) -> bool {
        match param {
            GlobalParameter::Oceans => self.oceans < (OCEANS_MAX_LEVEL - 1),
            GlobalParameter::Oxygen => self.oxygen < (OXYGEN_MAX_LEVEL - 1),
            GlobalParameter::Temperature => self.temperature < (TEMPERATURE_MAX_LEVEL - 1),
            GlobalParameter::Venus => self.venus < (VENUS_MAX_LEVEL - 1),
        }
    }

    /// Get the step size for a parameter
    pub fn step_size(param: GlobalParameter) -> i32 {
        match param {
            GlobalParameter::Oceans => OCEANS_STEP as i32,
            GlobalParameter::Oxygen => OXYGEN_STEP as i32,
            GlobalParameter::Temperature => TEMPERATURE_STEP,
            GlobalParameter::Venus => VENUS_STEP as i32,
        }
    }

    /// Validate that a value is on a valid step boundary for the parameter
    pub fn is_valid_step(param: GlobalParameter, value: i32) -> bool {
        match param {
            GlobalParameter::Oceans => {
                (0..=MAX_OCEANS as i32).contains(&value)
            }
            GlobalParameter::Oxygen => {
                (0..=MAX_OXYGEN as i32).contains(&value)
            }
            GlobalParameter::Temperature => {
                (MIN_TEMPERATURE..=MAX_TEMPERATURE).contains(&value)
                    && (value - MIN_TEMPERATURE) % TEMPERATURE_STEP == 0
            }
            GlobalParameter::Venus => {
                (0..=MAX_VENUS as i32).contains(&value) && value % (VENUS_STEP as i32) == 0
            }
        }
    }

    /// Deprecated: Use `increase()` instead. Global parameters can only increase.
    /// This method is kept for backward compatibility but only allows positive amounts.
    #[deprecated(note = "Use increase() instead. Global parameters can only increase.")]
    pub fn add(&mut self, param: GlobalParameter, amount: i32) {
        if amount <= 0 {
            return; // Can only increase
        }
        
        match param {
            GlobalParameter::Oceans => {
                let steps = amount as u32; // 1 step = 1 unit
                self.increase(param, steps);
            }
            GlobalParameter::Oxygen => {
                let steps = amount as u32; // 1 step = 1 unit
                self.increase(param, steps);
            }
            GlobalParameter::Temperature => {
                let steps = (amount + TEMPERATURE_STEP - 1) / TEMPERATURE_STEP; // Round up
                self.increase(param, steps as u32);
            }
            GlobalParameter::Venus => {
                let steps = (amount as u32).div_ceil(VENUS_STEP);
                self.increase(param, steps);
            }
        }
    }

    /// Check if Mars is fully terraformed (all parameters at max)
    pub fn is_fully_terraformed(&self) -> bool {
        self.oceans >= (OCEANS_MAX_LEVEL - 1)
            && self.oxygen >= (OXYGEN_MAX_LEVEL - 1)
            && self.temperature >= (TEMPERATURE_MAX_LEVEL - 1)
            && self.venus >= (VENUS_MAX_LEVEL - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_parameters() {
        let mut params = GlobalParameters::new();
        
        assert_eq!(params.get(GlobalParameter::Temperature), MIN_TEMPERATURE);
        assert_eq!(params.get(GlobalParameter::Oceans), 0);
        
        // Increase temperature by 5 steps (5 * 2 = 10 degrees)
        params.increase(GlobalParameter::Temperature, 5);
        assert_eq!(params.get(GlobalParameter::Temperature), -20); // -30 + 10 = -20
        
        // Increase oceans by 5 steps (5 * 1 = 5 oceans)
        let steps = params.increase(GlobalParameter::Oceans, 5);
        assert_eq!(steps, 5);
        assert_eq!(params.get(GlobalParameter::Oceans), 5);
    }

    #[test]
    fn test_global_parameter_limits() {
        let mut params = GlobalParameters::new();
        
        // Test max clamping
        params.increase(GlobalParameter::Oceans, 100);
        assert_eq!(params.get(GlobalParameter::Oceans), MAX_OCEANS as i32);
        
        params.increase(GlobalParameter::Oxygen, 100);
        assert_eq!(params.get(GlobalParameter::Oxygen), MAX_OXYGEN as i32);
        
        params.increase(GlobalParameter::Temperature, 100);
        assert_eq!(params.get(GlobalParameter::Temperature), MAX_TEMPERATURE);
        
        params.increase(GlobalParameter::Venus, 100);
        assert_eq!(params.get(GlobalParameter::Venus), MAX_VENUS as i32);
    }

    #[test]
    fn test_step_sizes() {
        let mut params = GlobalParameters::new();
        
        // Temperature: steps of 2
        params.increase(GlobalParameter::Temperature, 1);
        assert_eq!(params.get(GlobalParameter::Temperature), -28); // -30 + 2 = -28
        
        params.increase(GlobalParameter::Temperature, 1);
        assert_eq!(params.get(GlobalParameter::Temperature), -26); // -28 + 2 = -26
        
        // Oxygen: steps of 1
        params.increase(GlobalParameter::Oxygen, 1);
        assert_eq!(params.get(GlobalParameter::Oxygen), 1);
        
        params.increase(GlobalParameter::Oxygen, 1);
        assert_eq!(params.get(GlobalParameter::Oxygen), 2);
        
        // Venus: steps of 2
        params.increase(GlobalParameter::Venus, 1);
        assert_eq!(params.get(GlobalParameter::Venus), 2);
        
        params.increase(GlobalParameter::Venus, 1);
        assert_eq!(params.get(GlobalParameter::Venus), 4);
    }

    #[test]
    fn test_cannot_decrease() {
        let mut params = GlobalParameters::new();
        
        // Set to some value
        params.increase(GlobalParameter::Oceans, 5);
        assert_eq!(params.get(GlobalParameter::Oceans), 5);
        
        // Try to decrease using deprecated add (should be ignored)
        #[allow(deprecated)]
        params.add(GlobalParameter::Oceans, -10);
        assert_eq!(params.get(GlobalParameter::Oceans), 5); // Unchanged
        
        // Can only increase
        params.increase(GlobalParameter::Oceans, 1);
        assert_eq!(params.get(GlobalParameter::Oceans), 6);
    }

    #[test]
    fn test_valid_step_validation() {
        // Valid values
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Temperature, -30));
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Temperature, -28));
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Temperature, 0));
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Temperature, 8));
        
        // Invalid values (not on step boundary)
        assert!(!GlobalParameters::is_valid_step(GlobalParameter::Temperature, -29));
        assert!(!GlobalParameters::is_valid_step(GlobalParameter::Temperature, -1));
        assert!(!GlobalParameters::is_valid_step(GlobalParameter::Temperature, 1));
        
        // Valid Venus values (steps of 2)
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Venus, 0));
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Venus, 2));
        assert!(GlobalParameters::is_valid_step(GlobalParameter::Venus, 30));
        
        // Invalid Venus values
        assert!(!GlobalParameters::is_valid_step(GlobalParameter::Venus, 1));
        assert!(!GlobalParameters::is_valid_step(GlobalParameter::Venus, 3));
    }

    #[test]
    fn test_set_rounds_to_nearest_step() {
        let mut params = GlobalParameters::new();
        
        // Set temperature to -29 (should round to -28, nearest step of 2)
        params.set(GlobalParameter::Temperature, -29);
        assert_eq!(params.get(GlobalParameter::Temperature), -28);
        
        // Reset and test positive rounding
        params.set(GlobalParameter::Temperature, MIN_TEMPERATURE);
        
        // Set temperature to 1 (should round to 2, nearest step of 2)
        params.set(GlobalParameter::Temperature, 1);
        assert_eq!(params.get(GlobalParameter::Temperature), 2); // Rounds to 2
        
        // Set temperature to -1 (should round to 0)
        params.set(GlobalParameter::Temperature, -1);
        assert_eq!(params.get(GlobalParameter::Temperature), 0);
        
        // Set Venus to 1 (should round to 2)
        params.set(GlobalParameter::Venus, 1);
        assert_eq!(params.get(GlobalParameter::Venus), 2); // Rounds up
        
        // Set Venus to 3 (should round to 4)
        params.set(GlobalParameter::Venus, 3);
        assert_eq!(params.get(GlobalParameter::Venus), 4);
    }

    #[test]
    fn test_can_increase() {
        let mut params = GlobalParameters::new();
        
        assert!(params.can_increase(GlobalParameter::Oceans));
        assert!(params.can_increase(GlobalParameter::Oxygen));
        assert!(params.can_increase(GlobalParameter::Temperature));
        assert!(params.can_increase(GlobalParameter::Venus));
        
        // Set to max
        params.increase(GlobalParameter::Oceans, 100);
        assert!(!params.can_increase(GlobalParameter::Oceans));
        
        params.increase(GlobalParameter::Temperature, 100);
        assert!(!params.can_increase(GlobalParameter::Temperature));
    }

    #[test]
    fn test_increase_returns_actual_steps() {
        let mut params = GlobalParameters::new();
        
        // Normal increase - should return all requested steps
        let steps = params.increase(GlobalParameter::Oceans, 5);
        assert_eq!(steps, 5);
        assert_eq!(params.get(GlobalParameter::Oceans), 5);
        
        // Increase when near max - should return only available steps
        params.increase(GlobalParameter::Oceans, 3); // Now at 8 (max is 9, so 1 step left)
        let steps = params.increase(GlobalParameter::Oceans, 5); // Try to increase by 5
        assert_eq!(steps, 1); // Only 1 step was possible
        assert_eq!(params.get(GlobalParameter::Oceans), 9); // At max
        
        // Try to increase when at max - should return 0
        let steps = params.increase(GlobalParameter::Oceans, 10);
        assert_eq!(steps, 0);
        assert_eq!(params.get(GlobalParameter::Oceans), 9); // Still at max
        
        // Test with temperature
        params.set(GlobalParameter::Temperature, 6); // 1 step from max (8)
        let steps = params.increase(GlobalParameter::Temperature, 5);
        assert_eq!(steps, 1); // Only 1 step possible (6 -> 8)
        assert_eq!(params.get(GlobalParameter::Temperature), 8); // At max
    }
}

