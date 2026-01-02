"""
Terraforming Mars Simulation Engine - Python Bindings

This module provides Python bindings for the Rust simulation engine.
"""

# The actual module will be built by maturin
# For development, import from the built extension
try:
    from reinforcing_mars_sim._lib import (
        PyGame,
        PyPlayer,
        PyAction,
        PyPayment,
        PyPaymentMethod,
        PyPaymentReserve,
        PyStandardProjectParams,
        PyPhase,
    )
except ImportError:
    # If the extension hasn't been built yet, provide placeholder classes
    # This allows the package to be imported even before building
    pass

__version__ = "0.1.0"

