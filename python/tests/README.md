# Python Bindings Tests

Tests for the reinforcing-mars-sim Python bindings.

## Prerequisites

Before running tests, make sure:
1. The Python extension is built and installed
2. You're in the virtual environment (if using uv)

## Quick Start

```bash
# If using uv (recommended)
source .venv/bin/activate
make test

# Or manually
pytest python/tests/ -v
```

## Setup

### Using uv

```bash
# Create and activate virtual environment
uv venv
source .venv/bin/activate

# Install dependencies
uv pip install maturin pytest pytest-cov

# Build and install extension
cd simulation
maturin build --features pyo3
cd ..
uv pip install simulation/target/wheels/reinforcing_mars_sim-*.whl

# Run tests
pytest python/tests/ -v
```

### Using Makefile

```bash
# First time setup
make setup

# Build and install extension
make install

# Run tests
make test

# Or rebuild and test
make rebuild && make test
```

## Running Tests

```bash
# Run all tests
pytest python/tests/

# Run with verbose output
pytest python/tests/ -v

# Run specific test file
pytest python/tests/test_game_basic.py

# Run specific test
pytest python/tests/test_game_basic.py::test_game_creation

# Run with coverage
pytest python/tests/ --cov=reinforcing_mars_sim
```

## Test Structure

- `test_game_basic.py` - Basic game functionality tests
- `test_action_types.py` - Action type creation and validation
- `test_player.py` - Player wrapper tests
- `test_integration.py` - Integration tests for game flow
- `test_uv_integration.py` - Tests for uv installation
- `conftest.py` - Pytest fixtures

## Troubleshooting

### ModuleNotFoundError: No module named 'reinforcing_mars_sim'

The extension hasn't been installed. Run:
```bash
make install
# or
cd simulation && maturin build --features pyo3
cd .. && uv pip install simulation/target/wheels/*.whl
```

### Tests fail with import errors

Make sure you're in the virtual environment:
```bash
source .venv/bin/activate
```

### maturin not found

Install maturin:
```bash
uv pip install maturin
```
