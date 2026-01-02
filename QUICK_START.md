# Quick Start Guide

## First Time Setup

1. **Create virtual environment:**
   ```bash
   uv venv
   source .venv/bin/activate
   ```

2. **Install maturin:**
   ```bash
   uv pip install maturin
   ```

3. **Build and install the extension:**
   ```bash
   make install
   # or manually:
   cd simulation
   maturin build --features pyo3
   cd ..
   uv pip install simulation/target/wheels/reinforcing_mars_sim-*.whl
   ```

4. **Install test dependencies:**
   ```bash
   uv pip install pytest pytest-cov
   ```

5. **Run tests:**
   ```bash
   make test
   # or
   pytest python/tests/ -v
   ```

## Daily Development

1. **Activate environment:**
   ```bash
   source .venv/bin/activate
   ```

2. **Make changes to Rust code**

3. **Rebuild and test:**
   ```bash
   make rebuild  # Rebuilds and reinstalls
   make test     # Runs tests
   ```

## Troubleshooting

### ModuleNotFoundError when running tests

The extension needs to be installed. Run:
```bash
make install
```

The `make test` command will automatically install if needed, but you can also do it manually.

### maturin not found

Install it in your venv:
```bash
source .venv/bin/activate
uv pip install maturin
```

