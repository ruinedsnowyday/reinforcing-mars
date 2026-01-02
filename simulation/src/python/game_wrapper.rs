use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyType};
use crate::game::game::Game;
use crate::board::BoardType;
use crate::actions::Action;
use crate::player::resources::Resource;
use crate::python::types::{PyAction, PyPhase};
use crate::python::player_wrapper::PyPlayer;

/// Python wrapper for Game
#[pyclass]
pub struct PyGame {
    game: Game,
}

impl PyGame {
    /// Internal helper to create a new game
    fn create_game(
        num_players: usize,
        seed: u64,
        board_type: Option<&str>,
        corporate_era: Option<bool>,
        venus_next: Option<bool>,
        colonies: Option<bool>,
        prelude: Option<bool>,
        prelude2: Option<bool>,
        turmoil: Option<bool>,
        promos: Option<bool>,
        draft_variant: Option<bool>,
    ) -> PyResult<Self> {
        // Create player names
        let player_names: Vec<String> = (1..=num_players)
            .map(|i| format!("Player {}", i))
            .collect();

        // Parse board type
        let board = match board_type.unwrap_or("Tharsis") {
            "Tharsis" => BoardType::Tharsis,
            "Hellas" => BoardType::Hellas,
            "Elysium" => BoardType::Elysium,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown board type: {}", board_type.unwrap_or("Tharsis"))
            )),
        };

        let game = Game::new(
            format!("game_{}", seed),
            player_names,
            seed,
            board,
            corporate_era.unwrap_or(false),
            venus_next.unwrap_or(false),
            colonies.unwrap_or(false),
            prelude.unwrap_or(false),
            prelude2.unwrap_or(false),
            turmoil.unwrap_or(false),
            promos.unwrap_or(false),
            draft_variant.unwrap_or(false),
        );

        Ok(Self { game })
    }
}

#[pymethods]
impl PyGame {
    /// Create a new game (classmethod)
    #[classmethod]
    #[pyo3(signature = (num_players, seed, *, board_type="Tharsis", corporate_era=false, venus_next=false, colonies=false, prelude=false, prelude2=false, turmoil=false, promos=false, draft_variant=false))]
    fn new(
        _cls: &Bound<'_, PyType>,
        num_players: usize,
        seed: u64,
        board_type: Option<&str>,
        corporate_era: Option<bool>,
        venus_next: Option<bool>,
        colonies: Option<bool>,
        prelude: Option<bool>,
        prelude2: Option<bool>,
        turmoil: Option<bool>,
        promos: Option<bool>,
        draft_variant: Option<bool>,
    ) -> PyResult<Self> {
        Self::create_game(
            num_players,
            seed,
            board_type,
            corporate_era,
            venus_next,
            colonies,
            prelude,
            prelude2,
            turmoil,
            promos,
            draft_variant,
        )
    }

    /// Create a new game (constructor)
    #[new]
    #[pyo3(signature = (num_players, seed, *, board_type="Tharsis", corporate_era=false, venus_next=false, colonies=false, prelude=false, prelude2=false, turmoil=false, promos=false, draft_variant=false))]
    fn __new__(
        num_players: usize,
        seed: u64,
        board_type: Option<&str>,
        corporate_era: Option<bool>,
        venus_next: Option<bool>,
        colonies: Option<bool>,
        prelude: Option<bool>,
        prelude2: Option<bool>,
        turmoil: Option<bool>,
        promos: Option<bool>,
        draft_variant: Option<bool>,
    ) -> PyResult<Self> {
        Self::create_game(
            num_players,
            seed,
            board_type,
            corporate_era,
            venus_next,
            colonies,
            prelude,
            prelude2,
            turmoil,
            promos,
            draft_variant,
        )
    }

    /// Execute an action (step the game forward)
    fn step(&mut self, action: &PyAction) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            // Convert Python action to Rust action
            let rust_action = action.to_rust_action()?;
            
            // Execute the action
            self.game.execute_action(&rust_action)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
            
            // Return game state (simplified for now)
            let dict = PyDict::new_bound(py);
            dict.set_item("phase", PyPhase::from_rust_phase(&self.game.phase).phase)?;
            dict.set_item("generation", self.game.generation)?;
            Ok(dict.into())
        })
    }

    /// Get current observation (game state as numpy array or dict)
    fn get_observation(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        
        // Basic game state
        dict.set_item("phase", PyPhase::from_rust_phase(&self.game.phase).phase)?;
        dict.set_item("generation", self.game.generation)?;
        dict.set_item("active_player_id", self.game.active_player_id.as_ref().map(|s| s.as_str()).unwrap_or(""))?;
        
        // Players
        let players_list = PyList::empty_bound(py);
        for player in &self.game.players {
            let player_dict = PyDict::new_bound(py);
            player_dict.set_item("id", &player.id)?;
            player_dict.set_item("name", &player.name)?;
            player_dict.set_item("terraform_rating", player.terraform_rating)?;
            player_dict.set_item("victory_points", player.victory_points)?;
            
            // Resources
            let resources_dict = PyDict::new_bound(py);
            resources_dict.set_item("megacredits", player.resources.megacredits)?;
            resources_dict.set_item("steel", player.resources.steel)?;
            resources_dict.set_item("titanium", player.resources.titanium)?;
            resources_dict.set_item("plants", player.resources.plants)?;
            resources_dict.set_item("energy", player.resources.get(Resource::Energy))?;
            resources_dict.set_item("heat", player.resources.get(Resource::Heat))?;
            player_dict.set_item("resources", resources_dict)?;
            
            // Production
            let production_dict = PyDict::new_bound(py);
            production_dict.set_item("megacredits", player.production.megacredits)?;
            production_dict.set_item("steel", player.production.steel)?;
            production_dict.set_item("titanium", player.production.titanium)?;
            production_dict.set_item("plants", player.production.plants)?;
            production_dict.set_item("energy", player.production.energy)?;
            production_dict.set_item("heat", player.production.heat)?;
            player_dict.set_item("production", production_dict)?;
            
            // Cards
            player_dict.set_item("cards_in_hand", player.cards_in_hand.len())?;
            player_dict.set_item("played_cards", player.played_cards.len())?;
            
            players_list.append(player_dict)?;
        }
        dict.set_item("players", players_list)?;
        
        // Global parameters
        let global_params_dict = PyDict::new_bound(py);
        global_params_dict.set_item("oceans", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Oceans))?;
        global_params_dict.set_item("oxygen", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Oxygen))?;
        global_params_dict.set_item("temperature", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Temperature))?;
        if self.game.venus_next {
            global_params_dict.set_item("venus", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Venus))?;
        }
        dict.set_item("global_parameters", global_params_dict)?;
        
        Ok(dict.into())
    }

    /// Get valid actions for the current player
    fn get_valid_actions(&self, py: Python) -> PyResult<PyObject> {
        let actions_list = PyList::empty_bound(py);
        
        // Only return valid actions if we're in the action phase
        if self.game.phase != crate::game::phase::Phase::Action {
            // Return empty list if not in action phase
            return Ok(actions_list.into());
        }
        
        let player_id = match &self.game.active_player_id {
            Some(id) => id.clone(),
            None => return Ok(actions_list.into()),
        };
        
        // Always allow Pass
        let pass_action = PyAction::from_rust_action(&Action::Pass);
        actions_list.append(pass_action.into_py(py))?;
        
        // Check ConvertPlants
        if let Some(player) = self.game.get_player(&player_id) {
            if crate::actions::standard_actions::StandardActions::can_convert_plants(player).is_ok() {
                let convert_plants = PyAction::from_rust_action(&Action::ConvertPlants);
                actions_list.append(convert_plants.into_py(py))?;
            }
            
            // Check ConvertHeat
            if crate::actions::standard_actions::StandardActions::can_convert_heat(player).is_ok() {
                let convert_heat = PyAction::from_rust_action(&Action::ConvertHeat);
                actions_list.append(convert_heat.into_py(py))?;
            }
            
            // Add standard projects that can be executed
            for project_type in [
                crate::actions::action::StandardProjectType::SellPatents,
                crate::actions::action::StandardProjectType::PowerPlant,
                crate::actions::action::StandardProjectType::Asteroid,
                crate::actions::action::StandardProjectType::Aquifer,
                crate::actions::action::StandardProjectType::Greenery,
                crate::actions::action::StandardProjectType::City,
            ] {
                let params = crate::actions::action::StandardProjectParams::default();
                if crate::actions::standard_projects::StandardProjects::can_execute(project_type, player, &params).is_ok() {
                    let action = Action::StandardProject {
                        project_type,
                        payment: crate::actions::payment::Payment::default(),
                        params,
                    };
                    let py_action = PyAction::from_rust_action(&action);
                    actions_list.append(py_action.into_py(py))?;
                }
            }
            
            // Add cards in hand as playable actions (simplified - no payment validation)
            for card_id in &player.cards_in_hand {
                let action = Action::PlayCard {
                    card_id: card_id.clone(),
                    payment: crate::actions::payment::Payment::default(),
                };
                // Only add if it can be executed (basic validation)
                if crate::actions::action_executor::ActionExecutor::can_execute(&action, &self.game, &player_id).is_ok() {
                    let py_action = PyAction::from_rust_action(&action);
                    actions_list.append(py_action.into_py(py))?;
                }
            }
        }
        
        Ok(actions_list.into())
    }

    /// Check if game is terminal (ended)
    fn is_terminal(&self) -> bool {
        matches!(self.game.phase, crate::game::phase::Phase::End)
    }

    /// Get reward for the current player (for RL training)
    fn get_reward(&self, player_id: Option<&str>) -> PyResult<f32> {
        // For Phase 7, return a simple reward based on victory points
        // Full implementation will calculate proper rewards for RL
        if let Some(pid) = player_id {
            if let Some(player) = self.game.players.iter().find(|p| p.id == pid) {
                return Ok(player.victory_points as f32);
            }
        }
        Ok(0.0)
    }

    /// Get current phase
    fn get_phase(&self) -> String {
        format!("{:?}", self.game.phase)
    }

    /// Get current generation
    fn get_generation(&self) -> u32 {
        self.game.generation
    }

    /// Get active player ID
    fn get_active_player_id(&self) -> Option<String> {
        self.game.active_player_id.clone()
    }

    /// Get all players
    fn get_players(&self, py: Python) -> PyResult<PyObject> {
        let players_list = PyList::empty_bound(py);
        for player in &self.game.players {
            let py_player = PyPlayer::from_rust_player(player);
            players_list.append(py_player.into_py(py))?;
        }
        Ok(players_list.into())
    }

    /// Get a player by ID
    fn get_player(&self, player_id: &str) -> PyResult<PyPlayer> {
        let player_id_string = player_id.to_string();
        let player = self.game.get_player(&player_id_string)
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Player {} not found", player_id)
            ))?;
        Ok(PyPlayer::from_rust_player(player))
    }

    /// Get global parameters as a dict
    fn get_global_parameters(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("oceans", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Oceans))?;
        dict.set_item("oxygen", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Oxygen))?;
        dict.set_item("temperature", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Temperature))?;
        if self.game.venus_next {
            dict.set_item("venus", self.game.global_parameters.get(crate::game::global_params::GlobalParameter::Venus))?;
        }
        Ok(dict.into())
    }

    /// Check if action is valid
    fn is_action_valid(&self, action: &PyAction) -> PyResult<bool> {
        let rust_action = action.to_rust_action()?;
        let player_id = self.game.active_player_id
            .as_ref()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("No active player"))?;
        
        match crate::actions::action_executor::ActionExecutor::can_execute(&rust_action, &self.game, player_id) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Reset the game (for testing)
    fn reset(&mut self, seed: Option<u64>) -> PyResult<()> {
        let new_seed = seed.unwrap_or(self.game.rng_seed);
        let num_players = self.game.players.len();
        let player_names: Vec<String> = (1..=num_players)
            .map(|i| format!("Player {}", i))
            .collect();
        
        let board = self.game.board.board_type();
        
        self.game = Game::new(
            format!("game_{}", new_seed),
            player_names,
            new_seed,
            board,
            self.game.corporate_era,
            self.game.venus_next,
            self.game.colonies,
            self.game.prelude,
            self.game.prelude2,
            self.game.turmoil,
            self.game.promos,
            self.game.draft_variant,
        );
        
        Ok(())
    }
}

