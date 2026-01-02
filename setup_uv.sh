#!/bin/bash
# Quick setup script for uv

set -e

echo "Setting up reinforcing-mars with uv..."

# Check if uv is installed
if ! command -v uv &> /dev/null; then
    echo "Error: uv is not installed. Install it with:"
    echo "  curl -LsSf https://astral.sh/uv/install.sh | sh"
    exit 1
fi

# Create virtual environment
echo "Creating virtual environment..."
uv venv

# Activate virtual environment
echo "Activating virtual environment..."
source .venv/bin/activate

# Install maturin
echo "Installing maturin..."
uv pip install maturin

# Build the extension
echo "Building Python extension..."
cd simulation
maturin build --features pyo3

# Install the wheel
echo "Installing extension..."
cd ..
uv pip install simulation/target/wheels/reinforcing_mars_sim-*.whl

# Install test dependencies
echo "Installing test dependencies..."
uv pip install pytest pytest-cov

echo ""
echo "✅ Setup complete!"
echo ""
echo "To activate the environment in the future:"
echo "  source .venv/bin/activate"
echo ""
echo "To rebuild after code changes:"
echo "  cd simulation && maturin build --features pyo3"
echo "  cd .. && uv pip install --force-reinstall simulation/target/wheels/reinforcing_mars_sim-*.whl"
echo ""
echo "To run tests:"
echo "  pytest python/tests/"

