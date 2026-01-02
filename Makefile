.PHONY: setup build install install-only test clean help check-installed

help:
	@echo "Available targets:"
	@echo "  setup    - Set up uv environment and install dependencies"
	@echo "  build    - Build the Python extension"
	@echo "  install  - Install the built extension"
	@echo "  test     - Run Python tests (installs extension if needed)"
	@echo "  clean    - Clean build artifacts"
	@echo "  rebuild  - Rebuild and reinstall the extension"

setup:
	@echo "Setting up with uv..."
	uv venv
	@echo "Activate with: source .venv/bin/activate"
	@echo "Then run: uv pip install maturin pytest pytest-cov"

build:
	@echo "Building Python extension..."
	cd simulation && maturin build --features pyo3

install-only:
	@echo "Installing extension (assuming wheel exists)..."
	@if [ -d .venv ]; then \
		uv pip install --python .venv/bin/python --force-reinstall simulation/target/wheels/reinforcing_mars_sim-*.whl; \
	else \
		uv pip install --force-reinstall simulation/target/wheels/reinforcing_mars_sim-*.whl; \
	fi

install: build install-only

check-installed:
	@if [ -d .venv ]; then \
		if ! .venv/bin/python -c "import reinforcing_mars_sim" 2>/dev/null; then \
			echo "Extension not installed."; \
			if ls simulation/target/wheels/reinforcing_mars_sim-*.whl 1> /dev/null 2>&1; then \
				echo "Wheel found. Installing..."; \
				$(MAKE) install-only; \
			else \
				echo "No wheel found. Building and installing..."; \
				$(MAKE) install; \
			fi \
		fi \
	else \
		if ! python3 -c "import reinforcing_mars_sim" 2>/dev/null; then \
			echo "Extension not installed."; \
			if ls simulation/target/wheels/reinforcing_mars_sim-*.whl 1> /dev/null 2>&1; then \
				echo "Wheel found. Installing..."; \
				$(MAKE) install-only; \
			else \
				echo "No wheel found. Building and installing..."; \
				$(MAKE) install; \
			fi \
		fi \
	fi

test: check-installed
	@echo "Running tests..."
	@if [ -d .venv ]; then \
		.venv/bin/python -m pytest python/tests/ -v; \
	else \
		python3 -m pytest python/tests/ -v; \
	fi

clean:
	@echo "Cleaning build artifacts..."
	rm -rf simulation/target/wheels
	rm -rf simulation/target/maturin

