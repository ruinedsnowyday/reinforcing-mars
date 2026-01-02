# Using uv with reinforcing-mars

This project uses `uv` for Python package management. Here's how to set it up and use it.

## Prerequisites

Install `uv` if you haven't already:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
# or
pip install uv
```

## Setup

### 1. Create a virtual environment with uv

```bash
# From project root
uv venv
```

This creates a `.venv` directory in the project root.

### 2. Activate the virtual environment

```bash
# On macOS/Linux
source .venv/bin/activate

# On Windows
.venv\Scripts\activate
```

### 3. Install maturin in the environment

```bash
uv pip install maturin
```

## Building the Python Extension

### Recommended: Build wheel and install with uv

This is the most reliable method when using uv:

```bash
# Make sure .venv is activated
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

# Build the wheel
cd simulation
maturin build --features pyo3

# Install the wheel
cd ..
uv pip install simulation/target/wheels/reinforcing_mars_sim-*.whl
```

### Alternative: Use maturin develop (may have issues with uv)

If `maturin develop` works for you:

```bash
# Make sure .venv is activated
source .venv/bin/activate  # or .venv\Scripts\activate on Windows

cd simulation
maturin develop --features pyo3
```

**Note:** `maturin develop` may fail with uv if it can't detect the virtual environment. If this happens, use the build + install method above.

## Running Tests

```bash
# Make sure .venv is activated
source .venv/bin/activate

# Install test dependencies
uv pip install -e ".[dev]"  # If using project root pyproject.toml
# or
uv pip install pytest pytest-cov

# Run tests
pytest python/tests/
```

## Development Workflow

1. **Activate the environment:**
   ```bash
   source .venv/bin/activate
   ```

2. **Make changes to Rust code**

3. **Rebuild the extension:**
   ```bash
   cd simulation
   maturin build --features pyo3
   cd ..
   uv pip install --force-reinstall simulation/target/wheels/reinforcing_mars_sim-*.whl
   ```

4. **Run tests:**
   ```bash
   cd ..
   pytest python/tests/
   ```

## Troubleshooting

### maturin can't find Python

If `maturin develop` fails with "Couldn't find a virtualenv", make sure:
1. You've created the venv: `uv venv`
2. You've activated it: `source .venv/bin/activate`
3. The `VIRTUAL_ENV` environment variable is set (check with `echo $VIRTUAL_ENV`)

### Alternative: Use maturin build + uv pip install

If `maturin develop` continues to have issues, use the build + install workflow:

```bash
cd simulation
maturin build --features pyo3
cd ..
uv pip install simulation/target/wheels/reinforcing_mars_sim-*.whl
```

This builds a wheel file and installs it, which is more reliable when using uv.

## Project Structure

```
reinforcing-mars/
├── .venv/              # uv virtual environment (created by `uv venv`)
├── pyproject.toml      # Root project config (for uv)
├── simulation/
│   ├── pyproject.toml  # Maturin build config
│   ├── Cargo.toml       # Rust dependencies
│   └── src/            # Rust source code
└── python/
    ├── terraforming_mars/  # Python package
    └── tests/              # Python tests
```

