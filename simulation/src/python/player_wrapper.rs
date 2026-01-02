use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::player::Player;
use crate::player::resources::Resource;

/// Python wrapper for Player
#[pyclass]
pub struct PyPlayer {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub terraform_rating: i32,
    #[pyo3(get)]
    pub victory_points: i32,
    // Store player data for access
    resources_megacredits: u32,
    resources_steel: u32,
    resources_titanium: u32,
    resources_plants: u32,
    resources_energy: u32,
    resources_heat: u32,
    production_megacredits: i32,
    production_steel: u32,
    production_titanium: u32,
    production_plants: u32,
    production_energy: u32,
    production_heat: u32,
    cards_in_hand: Vec<String>,
    played_cards: Vec<String>,
}

#[pymethods]
impl PyPlayer {
    /// Get resources as a Python dict
    fn get_resources(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("megacredits", self.resources_megacredits)?;
        dict.set_item("steel", self.resources_steel)?;
        dict.set_item("titanium", self.resources_titanium)?;
        dict.set_item("plants", self.resources_plants)?;
        dict.set_item("energy", self.resources_energy)?;
        dict.set_item("heat", self.resources_heat)?;
        Ok(dict.into())
    }

    /// Get production as a Python dict
    fn get_production(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("megacredits", self.production_megacredits)?;
        dict.set_item("steel", self.production_steel)?;
        dict.set_item("titanium", self.production_titanium)?;
        dict.set_item("plants", self.production_plants)?;
        dict.set_item("energy", self.production_energy)?;
        dict.set_item("heat", self.production_heat)?;
        Ok(dict.into())
    }

    /// Get cards in hand
    fn get_cards_in_hand(&self) -> PyResult<Vec<String>> {
        Ok(self.cards_in_hand.clone())
    }

    /// Get played cards
    fn get_played_cards(&self) -> PyResult<Vec<String>> {
        Ok(self.played_cards.clone())
    }
}

impl PyPlayer {
    /// Create from Rust Player
    pub fn from_rust_player(player: &Player) -> Self {
        Self {
            id: player.id.clone(),
            name: player.name.clone(),
            terraform_rating: player.terraform_rating,
            victory_points: player.victory_points,
            resources_megacredits: player.resources.megacredits,
            resources_steel: player.resources.steel,
            resources_titanium: player.resources.titanium,
            resources_plants: player.resources.plants,
            resources_energy: player.resources.get(Resource::Energy),
            resources_heat: player.resources.get(Resource::Heat),
            production_megacredits: player.production.megacredits,
            production_steel: player.production.steel,
            production_titanium: player.production.titanium,
            production_plants: player.production.plants,
            production_energy: player.production.energy,
            production_heat: player.production.heat,
            cards_in_hand: player.cards_in_hand.clone(),
            played_cards: player.played_cards.clone(),
        }
    }
}

