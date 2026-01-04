use crate::player::{Player, PlayerId};
use crate::game::phase::Phase;
use crate::game::global_params::GlobalParameters;
use crate::game::milestones::{MilestoneData, ClaimedMilestone};
use crate::game::awards::{AwardData, FundedAward};
use crate::board::{Board, BoardType};
use crate::utils::random::SeededRandom;
use crate::actions::{Action, ActionExecutor};
use crate::deferred::{DeferredActionQueue, DeferredAction, DeferredActionResult};

/// Game struct - tracks game state
/// This is a skeleton implementation for Phase 1
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
    /// Game ID
    pub id: String,
    
    /// Players in the game
    pub players: Vec<Player>,
    
    /// Current phase
    pub phase: Phase,
    
    /// Current generation (starts at 1)
    pub generation: u32,
    
    /// Active player ID
    pub active_player_id: Option<PlayerId>,
    
    /// Players who have passed in the current action phase
    pub passed_players: Vec<PlayerId>,
    
    /// Actions taken by current active player this turn (1-2 max)
    pub actions_taken_this_turn: u32,
    
    /// Global parameters
    pub global_parameters: GlobalParameters,
    
    /// Board
    pub board: Board,
    
    /// RNG seed
    pub rng_seed: u64,
    
    /// Seeded random number generator
    #[serde(skip)]
    pub rng: SeededRandom,
    
    /// Expansion flags
    pub corporate_era: bool,
    pub venus_next: bool,
    pub colonies: bool,
    pub prelude: bool,
    pub prelude2: bool,
    pub turmoil: bool,
    pub promos: bool,
    
    /// Draft variant flag - if true, players draft cards in research phase
    pub draft_variant: bool,
    
    /// Milestones
    pub milestones: Vec<MilestoneData>,
    pub claimed_milestones: Vec<ClaimedMilestone>,
    
    /// Awards
    pub awards: Vec<AwardData>,
    pub funded_awards: Vec<FundedAward>,
    
    /// Solo mode flag
    pub solo_mode: bool,
    
    /// Neutral player (for solo mode)
    pub neutral_player: Option<Player>,
    
    /// Draft state: current draft round (1-based)
    pub draft_round: u32,
    
    /// Draft state: initial draft iteration (1, 2, or 3)
    /// 1 = first project card iteration
    /// 2 = second project card iteration
    /// 3 = prelude draft (if enabled)
    pub initial_draft_iteration: u32,
    
    /// Deferred action queue (Phase 6)
    /// Note: Cannot be serialized (contains trait objects)
    #[serde(skip)]
    pub deferred_actions: DeferredActionQueue,
}

impl Game {
    /// Create a new game
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        player_names: Vec<String>,
        rng_seed: u64,
        board_type: BoardType,
        corporate_era: bool,
        venus_next: bool,
        colonies: bool,
        prelude: bool,
        prelude2: bool,
        turmoil: bool,
        promos: bool,
        draft_variant: bool,
    ) -> Self {
        let solo_mode = player_names.len() == 1;
        
        let players: Vec<Player> = player_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                // Use player name as ID for Python API compatibility
                let mut player = Player::new(name.clone(), name);
                
                // Solo mode: player starts with 14 TR instead of 20
                if solo_mode {
                    player.terraform_rating = 14;
                }
                
                player
            })
            .collect();
        let neutral_player = if solo_mode {
            Some(Player::new("neutral".to_string(), "Neutral".to_string()))
        } else {
            None
        };
        
        let board = Board::new(board_type);
        let rng = SeededRandom::new(rng_seed);
        
        // Set first player as active
        let active_player_id = players.first().map(|p| p.id.clone());
        
        Self {
            id,
            players,
            phase: Phase::InitialDrafting,
            generation: 1,
            active_player_id,
            passed_players: Vec::new(),
            actions_taken_this_turn: 0,
            global_parameters: GlobalParameters::new(),
            board,
            rng_seed,
            rng,
            corporate_era,
            venus_next,
            colonies,
            prelude,
            prelude2,
            turmoil,
            promos,
            draft_variant,
            milestones: Vec::new(),
            claimed_milestones: Vec::new(),
            awards: Vec::new(),
            funded_awards: Vec::new(),
            solo_mode,
            neutral_player,
            draft_round: 1,
            initial_draft_iteration: 1,
            deferred_actions: DeferredActionQueue::new(),
        }
    }

    /// Get a player by ID
    pub fn get_player(&self, player_id: &PlayerId) -> Option<&Player> {
        self.players.iter().find(|p| p.id == *player_id)
    }

    /// Get a mutable player by ID
    pub fn get_player_mut(&mut self, player_id: &PlayerId) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == *player_id)
    }

    /// Check if Mars is fully terraformed
    pub fn is_fully_terraformed(&self) -> bool {
        self.global_parameters.is_fully_terraformed()
    }

    /// Check if game is in solo mode
    pub fn is_solo_mode(&self) -> bool {
        self.solo_mode
    }

    /// Transition to the next phase based on current game state
    /// Handles conditional transitions (preludes enabled, draft variant, etc.)
    pub fn next_phase(&mut self) -> Result<(), String> {
        let next_phase = match self.phase {
            Phase::InitialDrafting => {
                // Initial drafting always transitions to Research
                Phase::Research
            }
            Phase::Research => {
                // Research phase transitions based on generation and preludes
                if self.generation == 1 && self.prelude {
                    // Generation 1 with preludes: go to Preludes phase
                    Phase::Preludes
                } else {
                    // Generation 1 without preludes, or generation 2+: go to Action phase
                    Phase::Action
                }
            }
            Phase::Preludes => {
                // Preludes always transitions to Action
                Phase::Action
            }
            Phase::Drafting => {
                // Drafting always transitions to Research
                Phase::Research
            }
            Phase::Action => {
                // Action phase transitions to Production when all players pass
                // This should be called via end_action_phase() instead
                return Err("Action phase should be ended via end_action_phase() when all players pass".to_string());
            }
            Phase::Production => {
                // Production transitions to Solar only if Venus Next is enabled
                // Otherwise, skip directly to Intergeneration
                if self.venus_next {
                    Phase::Solar
                } else {
                    Phase::Intergeneration
                }
            }
            Phase::Solar => {
                // Solar always transitions to Intergeneration
                Phase::Intergeneration
            }
            Phase::Intergeneration => {
                // Intergeneration transitions based on draft variant
                if self.draft_variant {
                    // Draft variant: go to Drafting phase
                    Phase::Drafting
                } else {
                    // No-draft variant: go directly to Research phase
                    Phase::Research
                }
            }
            Phase::End => {
                return Err("Game has ended".to_string());
            }
        };

        self.phase = next_phase;
        Ok(())
    }

    /// End the action phase and transition to Production
    /// Should be called when all players have passed
    pub fn end_action_phase(&mut self) -> Result<(), String> {
        if self.phase != Phase::Action {
            return Err("Not in action phase".to_string());
        }

        if !self.all_players_passed() {
            return Err("Not all players have passed".to_string());
        }

        // Reset passed players for next action phase
        self.reset_passed_players();

        // Transition to Production phase
        self.phase = Phase::Production;
        Ok(())
    }

    /// Get the active player
    pub fn active_player(&self) -> Option<&Player> {
        self.active_player_id
            .as_ref()
            .and_then(|id| self.get_player(id))
    }

    /// Get the active player mutably
    pub fn active_player_mut(&mut self) -> Option<&mut Player> {
        let player_id = self.active_player_id.clone()?;
        self.get_player_mut(&player_id)
    }

    /// Move to the next player
    pub fn next_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        let current_index = self
            .active_player_id
            .as_ref()
            .and_then(|id| {
                self.players
                    .iter()
                    .position(|p| p.id == *id)
            })
            .unwrap_or(0);

        let next_index = (current_index + 1) % self.players.len();
        self.active_player_id = Some(self.players[next_index].id.clone());
    }

    /// Mark the current player as passed and move to next player
    /// If all players have passed, automatically transitions to Production phase
    pub fn pass_player(&mut self) -> Result<(), String> {
        if self.phase != Phase::Action {
            return Err("Not in action phase".to_string());
        }

        let player_id = self
            .active_player_id
            .as_ref()
            .ok_or("No active player")?;

        // Mark player as passed if not already passed
        if !self.passed_players.contains(player_id) {
            self.passed_players.push(player_id.clone());
        }

        // Check if all players have passed
        if self.all_players_passed() {
            // Transition to Production phase
            self.end_action_phase()?;
            return Ok(());
        }

        // Move to next non-passed player
        self.move_to_next_active_player();

        Ok(())
    }

    /// Move to the next player who hasn't passed yet
    /// Wraps around to find the first non-passed player
    fn move_to_next_active_player(&mut self) {
        if self.players.is_empty() {
            return;
        }

        let current_index = self
            .active_player_id
            .as_ref()
            .and_then(|id| {
                self.players
                    .iter()
                    .position(|p| p.id == *id)
            })
            .unwrap_or(0);

        // Try each player starting from the next one
        for offset in 1..=self.players.len() {
            let next_index = (current_index + offset) % self.players.len();
            let next_player_id = &self.players[next_index].id;

            // If this player hasn't passed, make them active
            if !self.passed_players.contains(next_player_id) {
                self.active_player_id = Some(next_player_id.clone());
                // Reset action count for new player
                self.actions_taken_this_turn = 0;
                return;
            }
        }

        // All players have passed (shouldn't happen, but handle gracefully)
        // This case is already handled in pass_player(), but just in case
    }

    /// Start the action phase
    /// Sets the active player to the first player and resets passed players
    pub fn start_action_phase(&mut self) -> Result<(), String> {
        if self.phase != Phase::Action {
            return Err("Not in action phase".to_string());
        }

        // Reset passed players
        self.reset_passed_players();
        
        // Reset action count
        self.actions_taken_this_turn = 0;

        // Set active player to first player
        if let Some(first_player) = self.players.first() {
            self.active_player_id = Some(first_player.id.clone());
        } else {
            return Err("No players in game".to_string());
        }

        // In solo mode, handle neutral player actions
        if self.solo_mode {
            self.handle_neutral_player_action()?;
        }

        Ok(())
    }

    /// Handle neutral player action in solo mode
    /// This is a placeholder that will be expanded when we implement full action system
    fn handle_neutral_player_action(&mut self) -> Result<(), String> {
        // TODO: Implement neutral player automatic actions
        // Neutral player should:
        // - Take actions automatically (standard projects, card plays, etc.)
        // - Pass when appropriate
        // - This will be expanded in Phase 4 when we implement the action system

        // For now, this is a placeholder
        Ok(())
    }

    /// Execute an action during the action phase
    /// 
    /// This is the main entry point for players to take actions during their turn.
    /// 
    /// Per rulebook: "You can choose to take 1 or 2 actions on your turn."
    /// 
    /// Returns:
    /// - `Ok(())` if action executed successfully
    /// - `Err(...)` if action is invalid or cannot be executed
    pub fn execute_action(&mut self, action: &Action) -> Result<(), String> {
        if self.phase != Phase::Action {
            return Err("Not in action phase".to_string());
        }

        // Phase 6: Check deferred action queue before allowing player actions
        // Execute deferred actions in priority order until queue is empty or an action needs input
        // 
        // We use process_deferred_actions which processes actions one at a time to avoid
        // borrow checker conflicts (pops action, executes it, re-inserts only if needed)
        self.process_deferred_actions()?;

        // Get active player ID (clone to avoid borrow issues)
        let player_id = self.active_player_id
            .as_ref()
            .ok_or("No active player")?
            .clone();

        // Handle Pass action specially
        if action.is_pass() {
            return self.pass_player();
        }

        // Check action limit (1-2 actions per turn)
        if self.actions_taken_this_turn >= 2 {
            return Err("Action limit reached: players can take at most 2 actions per turn".to_string());
        }

        // Execute the action
        ActionExecutor::execute(action, self, &player_id)?;

        // Increment action count
        self.actions_taken_this_turn += 1;

        Ok(())
    }
    
    /// Get number of actions taken by current active player this turn
    pub fn actions_taken_this_turn(&self) -> u32 {
        self.actions_taken_this_turn
    }
    
    /// Check if current player can take more actions
    pub fn can_take_action(&self) -> bool {
        self.actions_taken_this_turn < 2
    }

    /// Defer an action to be executed before player actions
    /// This is the main entry point for adding deferred actions
    pub fn defer(&mut self, action: Box<dyn DeferredAction>) {
        self.deferred_actions.push(action);
    }

    /// Check if there are deferred actions pending
    pub fn has_deferred_actions(&self) -> bool {
        !self.deferred_actions.is_empty()
    }

    /// Process deferred actions in priority order
    /// Executes all deferred actions that can be executed immediately
    /// Stops if an action needs player input
    /// Returns Ok(()) if all actions completed or queue is empty
    /// Returns Err(...) if an action needs input
    /// 
    /// This method processes actions one at a time to avoid borrow checker conflicts.
    /// It should be called from the game loop before allowing player actions.
    pub fn process_deferred_actions(&mut self) -> Result<(), String> {
        // Process actions until queue is empty or one needs input
        // We manually process one at a time to avoid borrow conflicts
        loop {
            if self.deferred_actions.is_empty() {
                break;
            }
            
            // Pop action from queue (this borrows deferred_actions mutably)
            let mut action = match self.deferred_actions.pop_next_action() {
                Some(action) => action,
                None => break,
            };
            
            // Execute the action (this borrows self mutably, but action is no longer in queue)
            let result = action.execute(self);
            
            // Handle result
            match result {
                Ok(DeferredActionResult::Completed) => {
                    // Action completed, continue with next
                    continue;
                }
                Ok(DeferredActionResult::NeedsInput) => {
                    // Action needs input, re-insert at front and stop
                    self.deferred_actions.push_front_action(action);
                    return Err("Deferred action needs player input".to_string());
                }
                Ok(DeferredActionResult::Remove) => {
                    // Action requested removal, continue
                    continue;
                }
                Err(e) => {
                    // Action failed, log and continue
                    eprintln!("Deferred action failed: {}", e);
                    continue;
                }
            }
        }
        
        Ok(())
    }

    /// Check if all players have passed
    pub fn all_players_passed(&self) -> bool {
        self.passed_players.len() >= self.players.len()
    }

    /// Reset passed players (for new action phase)
    pub fn reset_passed_players(&mut self) {
        self.passed_players.clear();
    }

    /// Automatically complete research phase by selecting first available options
    /// This is useful for testing and RL training where we want to skip manual selection
    pub fn auto_complete_research_phase(&mut self) -> Result<(), String> {
        if self.phase != Phase::Research {
            return Err("Not in research phase".to_string());
        }

        // For each player, auto-select first available options
        for player in &mut self.players {
            // Generation 1: need corporation and preludes (if enabled)
            if self.generation == 1 {
                // Select first corporation if none selected
                if player.selected_corporation.is_none() && !player.dealt_corporation_cards.is_empty() {
                    player.selected_corporation = Some(player.dealt_corporation_cards[0].clone());
                }
                
                // Select first 2 preludes if enabled and not selected
                if self.prelude && player.selected_preludes.len() < 2 && !player.dealt_prelude_cards.is_empty() {
                    let needed = 2 - player.selected_preludes.len();
                    for i in 0..needed.min(player.dealt_prelude_cards.len()) {
                        if !player.selected_preludes.contains(&player.dealt_prelude_cards[i]) {
                            player.selected_preludes.push(player.dealt_prelude_cards[i].clone());
                        }
                    }
                }
            }
            // For generation 2+, research phase is optional (no auto-selection needed)
        }

        // Complete research phase and transition
        self.complete_research_phase()?;
        Ok(())
    }

    /// Automatically progress to next phase if conditions are met
    /// Returns true if phase was advanced, false if conditions not met
    pub fn try_advance_phase(&mut self) -> Result<bool, String> {
        match self.phase {
            Phase::InitialDrafting => {
                // Auto-advance to Research
                self.next_phase()?;
                Ok(true)
            }
            Phase::Research => {
                // Check if all players have completed research
                if self.all_players_research_complete() {
                    self.complete_research_phase()?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Phase::Preludes => {
                // Check if all players have played preludes
                if self.all_players_played_preludes() {
                    self.complete_preludes_phase()?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Phase::Action => {
                // Action phase advances when all players pass (handled in pass_player)
                Ok(false)
            }
            Phase::Production => {
                // Auto-advance after production
                self.complete_production_phase()?;
                Ok(true)
            }
            Phase::Solar => {
                // Auto-advance solar phase
                self.execute_solar_phase()?;
                Ok(true)
            }
            Phase::Intergeneration => {
                // Auto-advance intergeneration
                self.complete_intergeneration_phase()?;
                Ok(true)
            }
            Phase::Drafting => {
                // Drafting phase needs manual completion
                Ok(false)
            }
            Phase::End => {
                Ok(false)
            }
        }
    }

    /// Complete intergeneration phase and advance to next generation
    pub fn complete_intergeneration_phase(&mut self) -> Result<(), String> {
        if self.phase != Phase::Intergeneration {
            return Err("Not in intergeneration phase".to_string());
        }

        // Increment generation
        self.generation += 1;

        // Transition to next phase based on draft variant
        if self.draft_variant {
            self.phase = Phase::Drafting;
        } else {
            self.phase = Phase::Research;
            // Start research phase
            self.start_research_phase()?;
        }

        Ok(())
    }

    /// Increment generation and reset for next generation
    pub fn increment_generation(&mut self) {
        self.generation += 1;
        // Reset player states for new generation
        self.reset_passed_players();
        // Clear draft state
        for player in &mut self.players {
            player.draft_hand.clear();
            player.drafted_cards.clear();
            player.needs_to_draft = false;
        }
        // Reset draft round counter
        self.draft_round = 1;
    }

    /// Execute the Intergeneration phase
    /// 
    /// Per plan requirements:
    /// 1. Check win conditions (before incrementing generation)
    /// 2. Increment generation
    /// 3. Reset player states (passed players, draft state, etc.)
    /// 4. Check win conditions again (after incrementing)
    /// 5. Calculate victory points for all players (for tracking/display)
    /// 6. Determine winner (if game ended)
    /// 7. Transition to next phase (Drafting or Research based on draft variant)
    /// 
    /// Win conditions:
    /// - Multiplayer: All global parameters maxed (oceans, oxygen, temperature, venus if enabled)
    /// - Solo mode: Player reaches TR63 OR all global parameters maxed
    /// 
    /// Returns:
    /// - `Ok(Some(win_condition))` if game ended
    /// - `Ok(None)` if game continues
    /// - `Err(...)` if not in intergeneration phase or phase transition failed
    pub fn execute_intergeneration_phase(&mut self) -> Result<Option<WinCondition>, String> {
        if self.phase != Phase::Intergeneration {
            return Err("Not in intergeneration phase".to_string());
        }

        // Step 1: Check win conditions before incrementing generation
        // This catches win conditions that occurred during the previous generation
        if let Some(win_condition) = self.check_win_conditions() {
            // Game is over, transition to End phase
            // Calculate final victory points and determine winner
            let _vps = self.calculate_victory_points();
            let _winner = self.determine_winner();
            // Note: VP and winner are calculated but not stored here
            // They will be calculated again at game end for final scoring
            
            self.phase = Phase::End;
            return Ok(Some(win_condition));
        }

        // Step 2: Increment generation and reset player states
        // This includes:
        // - Incrementing generation counter
        // - Resetting passed players
        // - Clearing draft state (draft_hand, drafted_cards, needs_to_draft)
        // - Resetting draft round counter
        self.increment_generation();

        // Step 3: Check win conditions again after incrementing
        // This catches win conditions that might have been triggered by generation increment
        // (though currently generation increment doesn't trigger win conditions)
        if let Some(win_condition) = self.check_win_conditions() {
            // Game is over, transition to End phase
            let _vps = self.calculate_victory_points();
            let _winner = self.determine_winner();
            
            self.phase = Phase::End;
            return Ok(Some(win_condition));
        }

        // Step 4: Calculate victory points for all players (for tracking/display)
        // This is done each generation to track VP progression
        // Note: VP calculation is currently basic (TR only), will be expanded in later phases
        let _vps = self.calculate_victory_points();
        // TODO: Store VP per generation for statistics (similar to TypeScript's globalsPerGeneration)

        // Step 5: Game continues, transition to next phase
        // - If draft variant: go to Drafting phase
        // - If no-draft variant: go directly to Research phase
        self.next_phase()?;

        Ok(None)
    }

    /// Execute the Solar phase (Venus Next expansion only)
    /// 
    /// Per official rulebook:
    /// STEP 1: Game End Check
    /// - Check if temperature, oxygen, and oceans are all maxed out
    /// - If so, game ends and final scoring begins (no further steps executed)
    /// 
    /// STEP 2: World Government Terraforming
    /// - First player (player order hasn't shifted) acts as World Government
    /// - Chooses a non-maxed global parameter and increases it one step, OR places an ocean tile
    /// - All bonuses go to WG (no TR or other bonuses given to first player)
    /// - Other cards may be triggered by this (e.g., Arctic Algae, Aphrodite corporation)
    /// 
    /// Note: This phase is only present when Venus Next expansion is enabled.
    /// If Venus Next is not enabled, Production phase transitions directly to Intergeneration.
    pub fn execute_solar_phase(&mut self) -> Result<Option<WinCondition>, String> {
        if self.phase != Phase::Solar {
            return Err("Not in solar phase".to_string());
        }

        if !self.venus_next {
            return Err("Solar phase is only available when Venus Next expansion is enabled".to_string());
        }

        // STEP 1: Game End Check
        // Check if temperature, oxygen, and oceans are all maxed out
        // If so, game ends and final scoring begins (no further steps executed)
        if self.is_mars_terraformed() {
            // Game is over - transition to End phase
            self.phase = Phase::End;
            return Ok(Some(WinCondition::Terraformed));
        }

        // STEP 2: World Government Terraforming
        // First player acts as World Government and chooses a non-maxed parameter to increase
        // or places an ocean tile. Bonuses go to WG (no TR or bonuses to first player).
        // TODO: This will be implemented when we have full action system in Phase 4.
        // For now, this is a placeholder that will be expanded to:
        // - Get first player
        // - Present options (increase temperature, oxygen, venus, or place ocean)
        // - Execute chosen action without giving bonuses to first player
        // - Trigger card effects (e.g., Arctic Algae, Aphrodite)
        
        // Placeholder: World Government terraforming will be implemented in Phase 4
        // when we have the action system and tile placement logic

        // Transition to Intergeneration phase
        self.next_phase()?;

        Ok(None)
    }

    /// Check if Mars is terraformed (temperature, oxygen, and oceans all maxed)
    /// This is used in Solar Phase Step 1 to check for game end
    /// Note: Venus is NOT checked here - only Mars parameters
    pub fn is_mars_terraformed(&self) -> bool {
        let oceans_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oceans,
        ) >= crate::game::global_params::MAX_OCEANS as i32;
        let oxygen_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oxygen,
        ) >= crate::game::global_params::MAX_OXYGEN as i32;
        let temperature_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Temperature,
        ) >= crate::game::global_params::MAX_TEMPERATURE;

        oceans_maxed && oxygen_maxed && temperature_maxed
    }

    /// Execute production phase: add production to resources
    /// 
    /// Per official rulebook:
    /// 1. First, all energy is converted into heat (move all resource cubes from energy to heat)
    /// 2. Then, all players receive new resources:
    ///    - Players get M€ according to their terraform rating plus any M€ production (which may be negative!)
    ///    - Players also get any other resources they have production of
    /// 3. Finally, remove player markers from used action cards (to mark they may be used again next generation)
    /// 
    /// Note: Production values persist across generations (they are NOT reset)
    pub fn execute_production_phase(&mut self) -> Result<(), String> {
        if self.phase != Phase::Production {
            return Err("Not in production phase".to_string());
        }

        // Process production for all players simultaneously
        for player in &mut self.players {
            let production = &player.production;
            let resources = &mut player.resources;
            let tr = player.terraform_rating;

            // Step 1: Convert all existing energy to heat FIRST (before adding production)
            // Per rulebook: "First, all energy is converted into heat (move all resource cubes from the energy box to the heat box)"
            let existing_energy = resources.get(crate::player::resources::Resource::Energy);
            if existing_energy > 0 {
                resources.add(
                    crate::player::resources::Resource::Heat,
                    existing_energy,
                );
                resources.set(
                    crate::player::resources::Resource::Energy,
                    0,
                );
            }

            // Step 2: Add production to resources
            // Per rulebook: "Secondly, all players receive new resources"
            // Add megacredits: production + TR (TR is always positive)
            // Note: M€ production may be negative!
            let mc_production = production.megacredits;
            if mc_production >= 0 {
                resources.add(
                    crate::player::resources::Resource::Megacredits,
                    (mc_production as u32) + (tr as u32),
                );
            } else {
                // Negative production: add TR first, then subtract
                resources.add(
                    crate::player::resources::Resource::Megacredits,
                    tr as u32,
                );
                resources.subtract(
                    crate::player::resources::Resource::Megacredits,
                    (-mc_production) as u32,
                );
            }

            // Add other production (all non-negative)
            resources.add(
                crate::player::resources::Resource::Steel,
                production.steel,
            );
            resources.add(
                crate::player::resources::Resource::Titanium,
                production.titanium,
            );
            resources.add(
                crate::player::resources::Resource::Plants,
                production.plants,
            );
            resources.add(
                crate::player::resources::Resource::Energy,
                production.energy,
            );
            resources.add(
                crate::player::resources::Resource::Heat,
                production.heat,
            );

            // Note: Energy production is added to the energy box and stays there
            // It will be converted to heat in the NEXT production phase

            // Step 3: Remove player markers from used action cards
            // This allows action cards to be used again next generation
            // TODO: Implement when action cards are added in Phase 4
            // For now, this is a placeholder - action cards will track usage state
            // and this step will reset that state for all players' action cards
        }

        // Handle neutral player production in solo mode
        if let Some(ref mut _neutral) = self.neutral_player {
            // Neutral player production logic
            // In solo mode, neutral player has 2 cities on the board
            // Neutral player gets production from those cities
            // For now, this is a placeholder - will be expanded when we implement tile placement
            // TODO: Calculate neutral player production based on placed tiles
            // Neutral player typically gets:
            // - Production from city tiles (if any cards give city production)
            // - This will be implemented when we have full tile placement and card system
            
            // For now, neutral player production is handled as a placeholder
            // The neutral player's production will be calculated based on:
            // - City tiles placed on the board
            // - Any cards that affect neutral player production
            // This will be expanded in Phase 4 when we implement actions and tile placement
        }

        Ok(())
    }

    /// Execute production phase and transition to Solar phase
    /// This is the main entry point for completing the production phase
    pub fn complete_production_phase(&mut self) -> Result<(), String> {
        // Execute production
        self.execute_production_phase()?;

        // Transition to Solar phase
        self.next_phase()?;

        Ok(())
    }

    /// Check win conditions
    pub fn check_win_conditions(&self) -> Option<WinCondition> {
        if self.solo_mode {
            // Solo mode: win if TR >= 63 OR all global parameters maxed
            if let Some(player) = self.players.first() {
                if player.terraform_rating >= 63 {
                    return Some(WinCondition::SoloTr63);
                }
            }
        }

        // Check if terraformed (all enabled global parameters maxed)
        let oceans_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oceans,
        ) >= crate::game::global_params::MAX_OCEANS as i32;
        let oxygen_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oxygen,
        ) >= crate::game::global_params::MAX_OXYGEN as i32;
        let temperature_maxed = self.global_parameters.get(
            crate::game::global_params::GlobalParameter::Temperature,
        ) >= crate::game::global_params::MAX_TEMPERATURE;
        let venus_maxed = if self.venus_next {
            self.global_parameters.get(
                crate::game::global_params::GlobalParameter::Venus,
            ) >= crate::game::global_params::MAX_VENUS as i32
        } else {
            true // Venus not required if expansion not enabled
        };

        if oceans_maxed && oxygen_maxed && temperature_maxed && venus_maxed {
            return Some(WinCondition::Terraformed);
        }

        None
    }

    /// Calculate victory points for all players
    /// Returns a vector of (player_id, victory_points) tuples
    pub fn calculate_victory_points(&self) -> Vec<(PlayerId, u32)> {
        self.players
            .iter()
            .map(|player| {
                // Basic VP calculation: TR + other sources
                // This will be expanded in later phases
                let vp = player.terraform_rating.max(0) as u32;

                // TODO: Add other VP sources (cards, milestones, awards, etc.)

                (player.id.clone(), vp)
            })
            .collect()
    }

    /// Determine the winner based on victory points
    /// Returns the player ID with highest VP, or None if tie
    /// Tie-breaker: highest TR
    pub fn determine_winner(&self) -> Option<PlayerId> {
        let vps = self.calculate_victory_points();
        if vps.is_empty() {
            return None;
        }

        // Find player with highest VP
        let (winner_id, winner_vp) = vps.iter().max_by_key(|(_, vp)| vp)?;

        // Check for ties
        let tied_players: Vec<_> = vps
            .iter()
            .filter(|(_, vp)| vp == winner_vp)
            .collect();

        if tied_players.len() == 1 {
            return Some(winner_id.clone());
        }

        // Tie-breaker: highest TR
        let winner = tied_players
            .iter()
            .max_by_key(|(id, _)| {
                self.get_player(id)
                    .map(|p| p.terraform_rating)
                    .unwrap_or(0)
            })?;

        Some(winner.0.clone())
    }
}

/// Win condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinCondition {
    /// Solo mode: player reached TR 63
    SoloTr63,
    /// All global parameters maxed (multiplayer or solo)
    Terraformed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.generation, 1);
        assert_eq!(game.phase, Phase::InitialDrafting);
        assert!(!game.is_solo_mode());
        assert!(game.active_player_id.is_some());
    }

    #[test]
    fn test_solo_mode() {
        let game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );
        
        assert!(game.is_solo_mode());
        assert!(game.neutral_player.is_some());
        // Solo mode: player should start with 14 TR
        assert_eq!(game.players[0].terraform_rating, 14);
    }

    #[test]
    fn test_phase_transitions() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Initial phase
        assert_eq!(game.phase, Phase::InitialDrafting);

        // Transition to Research
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Research);
    }

    #[test]
    fn test_phase_transitions_with_preludes() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, true, false, false, false, false, // prelude enabled
        );

        // Start in InitialDrafting
        assert_eq!(game.phase, Phase::InitialDrafting);

        // Transition to Research
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Research);

        // Generation 1 with preludes: should transition to Preludes
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Preludes);

        // Preludes should transition to Action
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Action);
    }

    #[test]
    fn test_phase_transitions_no_preludes() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false, // no prelude
        );

        // Start in InitialDrafting
        assert_eq!(game.phase, Phase::InitialDrafting);

        // Transition to Research
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Research);

        // Generation 1 without preludes: should transition directly to Action
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Action);
    }

    #[test]
    fn test_end_action_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Set phase to Action
        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        // Pass first player
        assert!(game.pass_player().is_ok());
        assert_eq!(game.phase, Phase::Action); // Still in action phase

        // Pass second player (last one) - should auto-transition
        assert!(game.pass_player().is_ok());
        assert_eq!(game.phase, Phase::Production); // Automatically transitioned

        // Test manual end_action_phase when all players have passed
        // (This is now redundant since pass_player() does it automatically,
        // but we test it for completeness)
        game.phase = Phase::Action;
        game.start_action_phase().unwrap();
        
        // Manually mark all players as passed
        game.passed_players.push("p1".to_string());
        game.passed_players.push("p2".to_string());
        
        // Now can end action phase manually
        assert!(game.end_action_phase().is_ok());
        assert_eq!(game.phase, Phase::Production);
    }

    #[test]
    fn test_end_action_phase_not_all_passed() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Set phase to Action
        game.phase = Phase::Action;

        // Only one player passed
        assert!(game.pass_player().is_ok());

        // Cannot end action phase yet
        assert!(game.end_action_phase().is_err());
    }

    #[test]
    fn test_production_to_solar_transition_venus_next() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, true, false, false, false, false, false, false, // venus_next enabled
        );

        game.phase = Phase::Production;
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Solar);
    }

    #[test]
    fn test_production_to_intergeneration_no_venus_next() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false, // venus_next disabled
        );

        game.phase = Phase::Production;
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Intergeneration); // Skips Solar phase
    }

    #[test]
    fn test_solar_to_intergeneration_transition() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Solar;
        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Intergeneration);
    }

    #[test]
    fn test_intergeneration_transition_draft_variant() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, true, // draft variant
        );

        game.phase = Phase::Intergeneration;
        game.generation = 2; // Not generation 1

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Drafting);
    }

    #[test]
    fn test_intergeneration_transition_no_draft_variant() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false, // no draft variant
        );

        game.phase = Phase::Intergeneration;
        game.generation = 2; // Not generation 1

        assert!(game.next_phase().is_ok());
        assert_eq!(game.phase, Phase::Research);
    }

    #[test]
    fn test_execute_intergeneration_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Intergeneration;
        game.generation = 1;

        // Should increment generation and transition to Research (no draft variant)
        let result = game.execute_intergeneration_phase();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None); // No win condition
        assert_eq!(game.generation, 2);
        assert_eq!(game.phase, Phase::Research);
    }

    #[test]
    fn test_execute_intergeneration_phase_win_condition() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Intergeneration;
        // Set TR to 63 (solo mode win condition)
        game.players[0].terraform_rating = 63;

        let result = game.execute_intergeneration_phase();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(WinCondition::SoloTr63));
        assert_eq!(game.phase, Phase::End);
    }

    #[test]
    fn test_execute_solar_phase_venus_next() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, true, false, false, false, false, false, false, // venus_next enabled
        );

        game.phase = Phase::Solar;
        // Mars not terraformed
        assert!(!game.is_mars_terraformed());

        let result = game.execute_solar_phase();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None); // No win condition, continues
        assert_eq!(game.phase, Phase::Intergeneration);
    }

    #[test]
    fn test_execute_solar_phase_game_end_mars_terraformed() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, true, false, false, false, false, false, false, // venus_next enabled
        );

        game.phase = Phase::Solar;
        // Max out Mars parameters
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oceans,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oxygen,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Temperature,
            100,
        );
        assert!(game.is_mars_terraformed());

        let result = game.execute_solar_phase();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(WinCondition::Terraformed));
        assert_eq!(game.phase, Phase::End);
    }

    #[test]
    fn test_execute_solar_phase_no_venus_next() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false, // venus_next disabled
        );

        game.phase = Phase::Solar;
        let result = game.execute_solar_phase();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Venus Next"));
    }

    #[test]
    fn test_is_mars_terraformed() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Initially not terraformed
        assert!(!game.is_mars_terraformed());

        // Max out oceans
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oceans,
            100,
        );
        assert!(!game.is_mars_terraformed()); // Still not terraformed

        // Max out oxygen
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oxygen,
            100,
        );
        assert!(!game.is_mars_terraformed()); // Still not terraformed

        // Max out temperature
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Temperature,
            100,
        );
        assert!(game.is_mars_terraformed()); // Now terraformed

        // Venus is not checked for Mars terraforming
        assert!(game.is_mars_terraformed());
    }

    #[test]
    fn test_next_player() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        let first_player_id = game.active_player_id.clone();
        assert!(first_player_id.is_some());

        // Move to next player
        game.next_player();
        assert_ne!(game.active_player_id, first_player_id);

        // Move again
        let second_player_id = game.active_player_id.clone();
        game.next_player();
        assert_ne!(game.active_player_id, second_player_id);
        assert_ne!(game.active_player_id, first_player_id);

        // Should wrap around
        game.next_player();
        assert_eq!(game.active_player_id, first_player_id);
    }

    #[test]
    fn test_generation_increment() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        assert_eq!(game.generation, 1);
        game.increment_generation();
        assert_eq!(game.generation, 2);
        game.increment_generation();
        assert_eq!(game.generation, 3);
    }

    #[test]
    fn test_production_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = 5;
        player.production.steel = 2;
        player.production.energy = 3;
        player.terraform_rating = 20;

        // Execute production
        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Should have: 5 (production) + 20 (TR) = 25 megacredits
        assert_eq!(player.resources.megacredits, 25);
        assert_eq!(player.resources.steel, 2);
        // Energy production (3) stays in energy box until next production phase
        assert_eq!(player.resources.energy, 3);
        assert_eq!(player.resources.heat, 0);
    }

    #[test]
    fn test_production_phase_negative_mc() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = -3; // Negative production
        player.terraform_rating = 20;

        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Should have: 20 (TR) - 3 (negative production) = 17 megacredits
        assert_eq!(player.resources.megacredits, 17);
    }

    #[test]
    fn test_production_phase_wrong_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Not in production phase
        game.phase = Phase::Action;
        assert!(game.execute_production_phase().is_err());
    }

    #[test]
    fn test_complete_production_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = 5;
        player.production.steel = 2;
        player.terraform_rating = 20;

        // Complete production phase (executes production and transitions)
        assert!(game.complete_production_phase().is_ok());

        // Should have received production
        let player = game.players.first().unwrap();
        assert_eq!(player.resources.megacredits, 25); // 5 + 20 TR
        assert_eq!(player.resources.steel, 2);

        // Should have transitioned to next phase (Solar if Venus Next, Intergeneration otherwise)
        // Since venus_next is false, should go to Intergeneration
        assert_eq!(game.phase, Phase::Intergeneration);
    }

    #[test]
    fn test_production_phase_multiple_players() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        // Set different production for each player
        game.players[0].production.megacredits = 5;
        game.players[0].production.steel = 2;
        game.players[0].terraform_rating = 20;

        game.players[1].production.megacredits = 3;
        game.players[1].production.titanium = 1;
        game.players[1].terraform_rating = 18;

        assert!(game.execute_production_phase().is_ok());

        // Player 1: 5 + 20 = 25 M€, 2 steel
        assert_eq!(game.players[0].resources.megacredits, 25);
        assert_eq!(game.players[0].resources.steel, 2);

        // Player 2: 3 + 18 = 21 M€, 1 titanium
        assert_eq!(game.players[1].resources.megacredits, 21);
        assert_eq!(game.players[1].resources.titanium, 1);
    }

    #[test]
    fn test_production_phase_energy_conversion() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.production.energy = 5;
        player.resources.energy = 3; // Existing energy
        player.terraform_rating = 20;

        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Existing energy (3) should be converted to heat
        // Newly produced energy (5) stays in energy box until next production phase
        assert_eq!(player.resources.energy, 5); // Newly produced energy remains
        assert_eq!(player.resources.heat, 3); // Only existing energy converted
    }

    #[test]
    fn test_production_phase_energy_regression_newly_produced_stays() {
        // Regression test: Newly produced energy must NOT be converted to heat immediately
        // Per rulebook: "First, all energy is converted into heat" (existing energy only)
        // Then production is added, and newly produced energy stays in energy box
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        // Set up: player has existing energy and energy production
        player.resources.energy = 4; // Existing energy (should be converted to heat)
        player.production.energy = 7; // Energy production (should stay in energy box)
        player.terraform_rating = 20;

        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Regression check: newly produced energy (7) must remain in energy box
        assert_eq!(
            player.resources.energy, 7,
            "Newly produced energy must stay in energy box, not be converted to heat"
        );
        // Only existing energy (4) should be converted to heat
        assert_eq!(
            player.resources.heat, 4,
            "Only existing energy should be converted to heat, not newly produced energy"
        );
    }

    #[test]
    fn test_production_phase_energy_regression_no_existing_energy() {
        // Regression test: When there's no existing energy, only newly produced energy exists
        // This should stay in the energy box
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.resources.energy = 0; // No existing energy
        player.production.energy = 6; // Energy production
        player.terraform_rating = 20;

        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Newly produced energy (6) should stay in energy box
        assert_eq!(
            player.resources.energy, 6,
            "Newly produced energy must stay in energy box"
        );
        // No heat should be produced (no existing energy to convert)
        assert_eq!(
            player.resources.heat, 0,
            "No heat should be produced when there's no existing energy"
        );
    }

    #[test]
    fn test_production_phase_solo_mode() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Production;

        let player = game.players.first_mut().unwrap();
        player.production.megacredits = 5;
        player.terraform_rating = 14; // Solo mode starts with 14 TR

        // Should handle neutral player (placeholder for now)
        assert!(game.execute_production_phase().is_ok());

        let player = game.players.first().unwrap();
        // Should have: 5 (production) + 14 (TR) = 19 megacredits
        assert_eq!(player.resources.megacredits, 19);
    }

    #[test]
    fn test_pass_player() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Set to action phase
        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let first_player_id = game.active_player_id.clone();

        assert!(!game.all_players_passed());
        assert!(game.pass_player().is_ok());
        assert_eq!(game.passed_players.len(), 1);
        assert!(!game.all_players_passed());
        
        // Should have moved to next player
        assert_ne!(game.active_player_id, first_player_id);

        // Pass second player (last one)
        assert!(game.pass_player().is_ok());
        
        // Should have automatically transitioned to Production phase
        assert_eq!(game.phase, Phase::Production);
        
        // Passed players should be reset for next action phase
        assert_eq!(game.passed_players.len(), 0);
    }

    #[test]
    fn test_start_action_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Set to action phase
        game.phase = Phase::Action;

        // Start action phase
        assert!(game.start_action_phase().is_ok());
        
        // Active player should be first player
        assert_eq!(game.active_player_id, Some("p1".to_string()));
        
        // Passed players should be empty
        assert!(game.passed_players.is_empty());
    }

    #[test]
    fn test_start_action_phase_wrong_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Not in action phase
        assert!(game.start_action_phase().is_err());
    }

    #[test]
    fn test_pass_player_auto_transition() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let first_player_id = game.active_player_id.clone().unwrap();

        // Pass first player
        assert!(game.pass_player().is_ok());
        assert_eq!(game.passed_players.len(), 1);
        assert_ne!(game.active_player_id, Some(first_player_id.clone()));
        assert_eq!(game.phase, Phase::Action); // Still in action phase

        // Pass second player
        assert!(game.pass_player().is_ok());
        assert_eq!(game.passed_players.len(), 2);
        assert_eq!(game.phase, Phase::Action); // Still in action phase

        // Pass third player (last one)
        assert!(game.pass_player().is_ok());
        
        // Should have automatically transitioned to Production phase
        assert_eq!(game.phase, Phase::Production);
        
        // Passed players should be reset for next action phase
        assert_eq!(game.passed_players.len(), 0);
    }

    #[test]
    fn test_pass_player_wrong_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Not in action phase
        assert!(game.pass_player().is_err());
    }

    #[test]
    fn test_move_to_next_active_player() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string(), "Player 3".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let first_player_id = game.active_player_id.clone().unwrap();

        // Manually mark first player as passed
        game.passed_players.push(first_player_id.clone());

        // Move to next active player
        game.move_to_next_active_player();

        // Should have moved to second player
        assert_eq!(game.active_player_id, Some("p2".to_string()));
    }

    #[test]
    fn test_action_phase_solo_mode() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;

        // Start action phase (should handle neutral player)
        assert!(game.start_action_phase().is_ok());
        
        // Active player should be the solo player
        assert_eq!(game.active_player_id, Some("p1".to_string()));
    }

    #[test]
    fn test_win_conditions() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Solo mode, TR < 63, not terraformed
        assert!(game.check_win_conditions().is_none());

        // Set TR to 63
        game.players[0].terraform_rating = 63;
        assert_eq!(
            game.check_win_conditions(),
            Some(WinCondition::SoloTr63)
        );

        // Reset and terraform
        game.players[0].terraform_rating = 20;
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oceans,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Oxygen,
            100,
        );
        game.global_parameters.increase(
            crate::game::global_params::GlobalParameter::Temperature,
            100,
        );
        assert_eq!(
            game.check_win_conditions(),
            Some(WinCondition::Terraformed)
        );
    }

    #[test]
    fn test_victory_points() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.players[0].terraform_rating = 25;
        game.players[1].terraform_rating = 30;

        let vps = game.calculate_victory_points();
        assert_eq!(vps.len(), 2);
        assert!(vps.iter().any(|(id, vp)| id == "p1" && *vp == 25));
        assert!(vps.iter().any(|(id, vp)| id == "p2" && *vp == 30));

        // Player 2 should win
        assert_eq!(game.determine_winner(), Some("p2".to_string()));
    }

    #[test]
    fn test_execute_action_pass() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string(), "Player 2".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let pass_action = Action::Pass;
        assert!(game.execute_action(&pass_action).is_ok());
        // Should have moved to next player
        assert_ne!(game.active_player_id, Some("p1".to_string()));
    }

    #[test]
    fn test_execute_action_wrong_phase() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Research;
        let pass_action = Action::Pass;
        assert!(game.execute_action(&pass_action).is_err());
    }

    #[test]
    fn test_execute_action_convert_heat() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Heat, 8);
        let initial_tr = player.terraform_rating;

        let convert_heat_action = Action::ConvertHeat;
        assert!(game.execute_action(&convert_heat_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.get(crate::player::resources::Resource::Heat), 0);
        assert_eq!(player.terraform_rating, initial_tr + 1);
    }

    #[test]
    fn test_execute_action_convert_plants() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Plants, 8);
        let initial_oxygen = game.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oxygen,
        );

        let convert_plants_action = Action::ConvertPlants;
        assert!(game.execute_action(&convert_plants_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.get(crate::player::resources::Resource::Plants), 0);
        // Oxygen should have increased
        assert_eq!(
            game.global_parameters.get(crate::game::global_params::GlobalParameter::Oxygen),
            initial_oxygen + 1
        );
    }

    #[test]
    fn test_execute_action_sell_patents() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.add_card_to_hand("card1".to_string());
        player.add_card_to_hand("card2".to_string());
        let initial_mc = player.resources.megacredits;

        let sell_patents_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::SellPatents,
            payment: crate::actions::payment::Payment::default(),
            params: crate::actions::action::StandardProjectParams {
                card_ids: vec!["card1".to_string(), "card2".to_string()],
            },
        };
        assert!(game.execute_action(&sell_patents_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 2); // 1 M€ per card
    }

    #[test]
    fn test_execute_action_power_plant() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 11);
        let initial_energy_prod = player.production.energy;

        let power_plant_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::PowerPlant,
            payment: crate::actions::payment::Payment::with_megacredits(11),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(game.execute_action(&power_plant_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
        assert_eq!(player.production.energy, initial_energy_prod + 1);
    }

    #[test]
    fn test_execute_action_asteroid() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 14);
        let initial_temp = game.global_parameters.get(
            crate::game::global_params::GlobalParameter::Temperature,
        );

        let asteroid_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::Asteroid,
            payment: crate::actions::payment::Payment::with_megacredits(14),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(game.execute_action(&asteroid_action).is_ok());

        // Temperature should have increased
        // Note: Temperature increases in steps of 2, so 1 step = +2 temperature
        assert_eq!(
            game.global_parameters.get(crate::game::global_params::GlobalParameter::Temperature),
            initial_temp + 2 // 1 step = +2 temperature
        );
    }

    #[test]
    fn test_execute_action_aquifer() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 18);
        let initial_oceans = game.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oceans,
        );

        let aquifer_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::Aquifer,
            payment: crate::actions::payment::Payment::with_megacredits(18),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(game.execute_action(&aquifer_action).is_ok());

        // Oceans should have increased
        assert_eq!(
            game.global_parameters.get(crate::game::global_params::GlobalParameter::Oceans),
            initial_oceans + 1
        );
    }

    #[test]
    fn test_execute_action_greenery() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 23);
        let initial_oxygen = game.global_parameters.get(
            crate::game::global_params::GlobalParameter::Oxygen,
        );

        let greenery_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::Greenery,
            payment: crate::actions::payment::Payment::with_megacredits(23),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(game.execute_action(&greenery_action).is_ok());

        // Oxygen should have increased
        assert_eq!(
            game.global_parameters.get(crate::game::global_params::GlobalParameter::Oxygen),
            initial_oxygen + 1
        );
    }

    #[test]
    fn test_execute_action_city() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 25);

        let city_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::City,
            payment: crate::actions::payment::Payment::with_megacredits(25),
            params: crate::actions::action::StandardProjectParams::default(),
        };
        assert!(game.execute_action(&city_action).is_ok());

        // City placement doesn't change global parameters (just places tile)
        // Payment should be deducted
        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
    }

    #[test]
    fn test_execute_action_sell_patents_zero_cards() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        // Empty hand
        let sell_patents_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::SellPatents,
            payment: crate::actions::payment::Payment::default(),
            params: crate::actions::action::StandardProjectParams {
                card_ids: vec![],
            },
        };
        assert!(game.execute_action(&sell_patents_action).is_err());
    }

    #[test]
    fn test_execute_action_sell_patents_one_card() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.add_card_to_hand("card1".to_string());
        let initial_mc = player.resources.megacredits;

        let sell_patents_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::SellPatents,
            payment: crate::actions::payment::Payment::default(),
            params: crate::actions::action::StandardProjectParams {
                card_ids: vec!["card1".to_string()],
            },
        };
        assert!(game.execute_action(&sell_patents_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 1); // 1 M€ per card
    }

    #[test]
    fn test_execute_action_sell_patents_all_cards() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.add_card_to_hand("card1".to_string());
        player.add_card_to_hand("card2".to_string());
        player.add_card_to_hand("card3".to_string());
        let initial_mc = player.resources.megacredits;

        // Discard all cards
        let sell_patents_action = Action::StandardProject {
            project_type: crate::actions::action::StandardProjectType::SellPatents,
            payment: crate::actions::payment::Payment::default(),
            params: crate::actions::action::StandardProjectParams {
                card_ids: vec!["card1".to_string(), "card2".to_string(), "card3".to_string()],
            },
        };
        assert!(game.execute_action(&sell_patents_action).is_ok());

        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.cards_in_hand.len(), 0);
        assert_eq!(player.resources.megacredits, initial_mc + 3); // 3 M€ for 3 cards
    }

    #[test]
    fn test_execute_action_milestone_claiming() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add a milestone
        game.milestones.push(crate::game::milestones::MilestoneData {
            name: "test_milestone".to_string(),
            cost: 8,
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 8);

        // Claim milestone
        let action = Action::ClaimMilestone {
            milestone_id: "test_milestone".to_string(),
            payment: crate::actions::payment::Payment::with_megacredits(8),
        };
        assert!(game.execute_action(&action).is_ok());

        // Verify milestone was claimed
        assert_eq!(game.claimed_milestones.len(), 1);
        assert_eq!(game.claimed_milestones[0].player_id, "p1");
        assert_eq!(game.claimed_milestones[0].milestone_name, "test_milestone");

        // Verify payment was deducted
        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
    }

    #[test]
    fn test_execute_action_award_funding() {
        let mut game = Game::new(
            "game1".to_string(),
            vec!["Player 1".to_string()],
            12345,
            BoardType::Tharsis,
            false, false, false, false, false, false, false, false,
        );

        // Add an award
        game.awards.push(crate::game::awards::AwardData {
            name: "test_award".to_string(),
            funding_cost: 8,
        });

        game.phase = Phase::Action;
        game.start_action_phase().unwrap();

        let player = game.get_player_mut(&"p1".to_string()).unwrap();
        player.resources.add(crate::player::resources::Resource::Megacredits, 8);

        // Fund award
        let action = Action::FundAward {
            award_id: "test_award".to_string(),
            payment: crate::actions::payment::Payment::with_megacredits(8),
        };
        assert!(game.execute_action(&action).is_ok());

        // Verify award was funded
        assert_eq!(game.funded_awards.len(), 1);
        assert_eq!(game.funded_awards[0].player_id, "p1");
        assert_eq!(game.funded_awards[0].award_name, "test_award");

        // Verify payment was deducted
        let player = game.get_player(&"p1".to_string()).unwrap();
        assert_eq!(player.resources.megacredits, 0);
    }
}

