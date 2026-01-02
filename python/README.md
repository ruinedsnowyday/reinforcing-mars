# Terraforming Mars Python Bindings

Python bindings for the Rust Terraforming Mars simulation engine.

## Building

To build the Python extension, you need:
- Rust toolchain
- Python 3.8+
- maturin
- uv (recommended) or virtualenv/conda

### Using uv (Recommended)

See [UV_SETUP.md](UV_SETUP.md) for detailed instructions.

Quick start:
```bash
# Create virtual environment
uv venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Install maturin
uv pip install maturin

# Build and install extension
cd simulation
maturin build --features pyo3
cd ..
uv pip install simulation/target/wheels/*.whl
```

### Using virtualenv/conda

Install maturin:
```bash
pip install maturin
```

Build the extension:
```bash
cd simulation
maturin develop --features pyo3  # For development
# or
maturin build --features pyo3   # For distribution
```

## Quick Start with uv

```bash
# Run the setup script
./setup_uv.sh

# Or manually:
uv venv
source .venv/bin/activate
uv pip install maturin
cd simulation && maturin build --features pyo3
cd .. && uv pip install simulation/target/wheels/*.whl
```

See [UV_SETUP.md](UV_SETUP.md) for detailed instructions.

## Usage

```python
from reinforcing_mars_sim import PyGame, PyAction

# Create a new game
game = PyGame.new(num_players=2, seed=12345)

# Get observation
obs = game.get_observation()

# Execute an action
action = PyAction("Pass")
game.step(action)

# Check if terminal
if game.is_terminal():
    reward = game.get_reward("Player 1")
```

## API Reference

### PyGame

- `new(num_players, seed, **kwargs)` - Create a new game
- `step(action)` - Execute an action and return new state
- `get_observation()` - Get current game state as dict
- `get_valid_actions()` - Get list of valid actions
- `is_terminal()` - Check if game has ended
- `get_reward(player_id)` - Get reward for a player
- `get_phase()` - Get current phase
- `get_generation()` - Get current generation
- `get_active_player_id()` - Get active player ID
- `get_players()` - Get all players
- `get_player(player_id)` - Get a specific player

### PyAction

- `new(action_type)` - Create a new action
- `to_rust_action()` - Convert to Rust Action
- `from_rust_action(action)` - Create from Rust Action

### PyPlayer

- `get_resources()` - Get player resources
- `get_production()` - Get player production
- `get_cards_in_hand()` - Get cards in hand
- `get_played_cards()` - Get played cards

