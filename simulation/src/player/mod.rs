pub mod resources;
pub mod tags;
pub mod production;
#[allow(clippy::module_inception)]
pub mod player;

pub use player::PlayerId;
pub use player::Player;

