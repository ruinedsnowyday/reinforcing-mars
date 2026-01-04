// Terraforming Mars Simulation Engine
// Python bindings entry point

pub mod game;
pub mod player;
pub mod board;
pub mod cards;
pub mod actions;
pub mod deferred;
pub mod utils;

#[cfg(feature = "pyo3")]
pub mod python;

// Python module initialization
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
use python::{PyGame, PyPlayer, PyAction, PyPayment, PyPaymentMethod, PyPaymentReserve, PyStandardProjectParams, PyPhase};

/// Python module definition
#[cfg(feature = "pyo3")]
#[pymodule]
fn reinforcing_mars_sim(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGame>()?;
    m.add_class::<PyPlayer>()?;
    m.add_class::<PyAction>()?;
    m.add_class::<PyPayment>()?;
    m.add_class::<PyPaymentMethod>()?;
    m.add_class::<PyPaymentReserve>()?;
    m.add_class::<PyStandardProjectParams>()?;
    m.add_class::<PyPhase>()?;
    Ok(())
}

