/// Game phases in Terraforming Mars
/// 
/// Note: Ceos phase is unofficial and should NOT be included.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Phase {
    /// Initial drafting phase (optional variant)
    /// Includes project cards and prelude cards drafting
    InitialDrafting,
    
    /// Preludes phase - players play their selected prelude cards
    Preludes,
    
    /// Research phase - players select cards to keep
    /// In generation 1: includes corporation selection, prelude selection, project card selection
    /// In subsequent generations: players select cards from drafted/dealt cards
    Research,
    
    /// Drafting phase - standard drafting variant for subsequent generations
    Drafting,
    
    /// Action phase - players take actions
    Action,
    
    /// Production phase - production is added to resources
    Production,
    
    /// Solar phase - World Government terraforming and final greenery placement
    Solar,
    
    /// Intergeneration phase - cleanup and generation increment
    Intergeneration,
    
    /// End phase - game is over
    End,
}

impl Phase {
    /// Get the next phase in the normal game flow
    pub fn next(&self) -> Option<Phase> {
        match self {
            Phase::InitialDrafting => Some(Phase::Research),
            Phase::Research => Some(Phase::Preludes), // Or Action if no preludes
            Phase::Preludes => Some(Phase::Action),
            Phase::Drafting => Some(Phase::Research),
            Phase::Action => Some(Phase::Production),
            Phase::Production => Some(Phase::Solar),
            Phase::Solar => Some(Phase::Intergeneration),
            Phase::Intergeneration => Some(Phase::Research), // Or Drafting if draft variant
            Phase::End => None,
        }
    }
}

