/// Card types in Terraforming Mars
/// 
/// Note: Ceo card type is unofficial and should NOT be included.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    /// Automated cards - effects trigger automatically
    Automated,
    /// Active cards - have actions that can be activated
    Active,
    /// Event cards - one-time effects, then discarded
    Event,
    /// Prelude cards - played at game start
    Prelude,
    /// Corporation cards - starting corporations
    Corporation,
    /// Standard project cards - available to all players
    StandardProject,
    /// Standard action cards - available to all players
    StandardAction,
}

impl CardType {
    pub fn all() -> Vec<CardType> {
        vec![
            CardType::Automated,
            CardType::Active,
            CardType::Event,
            CardType::Prelude,
            CardType::Corporation,
            CardType::StandardProject,
            CardType::StandardAction,
        ]
    }
}

