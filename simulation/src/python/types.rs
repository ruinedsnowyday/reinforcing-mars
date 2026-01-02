use pyo3::prelude::*;
use crate::actions::action::{Action, StandardProjectType, StandardProjectParams};
use crate::actions::payment::{Payment, PaymentMethod, PaymentReserve};
use crate::game::phase::Phase;

/// Python-compatible Action enum
#[pyclass]
#[derive(Clone)]
pub struct PyAction {
    #[pyo3(get, set)]
    pub action_type: String,
    #[pyo3(get, set)]
    pub card_id: Option<String>,
    #[pyo3(get, set)]
    pub payment: Option<PyPayment>,
    #[pyo3(get, set)]
    pub project_type: Option<String>,
    #[pyo3(get, set)]
    pub params: Option<PyStandardProjectParams>,
    #[pyo3(get, set)]
    pub award_id: Option<String>,
    #[pyo3(get, set)]
    pub milestone_id: Option<String>,
}

#[pymethods]
impl PyAction {
    #[new]
    fn new(action_type: String) -> Self {
        Self {
            action_type,
            card_id: None,
            payment: None,
            project_type: None,
            params: None,
            award_id: None,
            milestone_id: None,
        }
    }
}

impl PyAction {
    /// Convert to Rust Action
    pub fn to_rust_action(&self) -> PyResult<Action> {
        match self.action_type.as_str() {
            "Pass" => Ok(Action::Pass),
            "ConvertPlants" => Ok(Action::ConvertPlants),
            "ConvertHeat" => Ok(Action::ConvertHeat),
            "PlayCard" => {
                let card_id = self.card_id.clone()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("card_id required for PlayCard"))?;
                let payment = self.payment.as_ref()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("payment required for PlayCard"))?
                    .to_rust_payment()?;
                Ok(Action::PlayCard { card_id, payment })
            }
            "StandardProject" => {
                let project_type_str = self.project_type.as_ref()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("project_type required for StandardProject"))?;
                let project_type = match project_type_str.as_str() {
                    "SellPatents" => StandardProjectType::SellPatents,
                    "PowerPlant" => StandardProjectType::PowerPlant,
                    "Asteroid" => StandardProjectType::Asteroid,
                    "Aquifer" => StandardProjectType::Aquifer,
                    "Greenery" => StandardProjectType::Greenery,
                    "City" => StandardProjectType::City,
                    _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Unknown project type: {}", project_type_str)
                    )),
                };
                let payment = self.payment.as_ref()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("payment required for StandardProject"))?
                    .to_rust_payment()?;
                let params = self.params.as_ref()
                    .map(|p| p.to_rust_params())
                    .unwrap_or_else(|| StandardProjectParams::default());
                Ok(Action::StandardProject { project_type, payment, params })
            }
            "FundAward" => {
                let award_id = self.award_id.clone()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("award_id required for FundAward"))?;
                let payment = self.payment.as_ref()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("payment required for FundAward"))?
                    .to_rust_payment()?;
                Ok(Action::FundAward { award_id, payment })
            }
            "ClaimMilestone" => {
                let milestone_id = self.milestone_id.clone()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("milestone_id required for ClaimMilestone"))?;
                let payment = self.payment.as_ref()
                    .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("payment required for ClaimMilestone"))?
                    .to_rust_payment()?;
                Ok(Action::ClaimMilestone { milestone_id, payment })
            }
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown action type: {}", self.action_type)
            )),
        }
    }

    /// Create from Rust Action
    pub fn from_rust_action(action: &Action) -> Self {
        match action {
            Action::Pass => Self {
                action_type: "Pass".to_string(),
                card_id: None,
                payment: None,
                project_type: None,
                params: None,
                award_id: None,
                milestone_id: None,
            },
            Action::ConvertPlants => Self {
                action_type: "ConvertPlants".to_string(),
                card_id: None,
                payment: None,
                project_type: None,
                params: None,
                award_id: None,
                milestone_id: None,
            },
            Action::ConvertHeat => Self {
                action_type: "ConvertHeat".to_string(),
                card_id: None,
                payment: None,
                project_type: None,
                params: None,
                award_id: None,
                milestone_id: None,
            },
            Action::PlayCard { card_id, payment } => Self {
                action_type: "PlayCard".to_string(),
                card_id: Some(card_id.clone()),
                payment: Some(PyPayment::from_rust_payment(payment)),
                project_type: None,
                params: None,
                award_id: None,
                milestone_id: None,
            },
            Action::StandardProject { project_type, payment, params } => Self {
                action_type: "StandardProject".to_string(),
                card_id: None,
                payment: Some(PyPayment::from_rust_payment(payment)),
                project_type: Some(match project_type {
                    StandardProjectType::SellPatents => "SellPatents".to_string(),
                    StandardProjectType::PowerPlant => "PowerPlant".to_string(),
                    StandardProjectType::Asteroid => "Asteroid".to_string(),
                    StandardProjectType::Aquifer => "Aquifer".to_string(),
                    StandardProjectType::Greenery => "Greenery".to_string(),
                    StandardProjectType::City => "City".to_string(),
                }),
                params: Some(PyStandardProjectParams::from_rust_params(params)),
                award_id: None,
                milestone_id: None,
            },
            Action::FundAward { award_id, payment } => Self {
                action_type: "FundAward".to_string(),
                card_id: None,
                payment: Some(PyPayment::from_rust_payment(payment)),
                project_type: None,
                params: None,
                award_id: Some(award_id.clone()),
                milestone_id: None,
            },
            Action::ClaimMilestone { milestone_id, payment } => Self {
                action_type: "ClaimMilestone".to_string(),
                card_id: None,
                payment: Some(PyPayment::from_rust_payment(payment)),
                project_type: None,
                params: None,
                award_id: None,
                milestone_id: Some(milestone_id.clone()),
            },
        }
    }
}

/// Python-compatible Payment
#[pyclass]
#[derive(Clone)]
pub struct PyPayment {
    #[pyo3(get, set)]
    pub methods: Vec<PyPaymentMethod>,
    #[pyo3(get, set)]
    pub reserve: PyPaymentReserve,
}

#[pymethods]
impl PyPayment {
    #[new]
    fn new() -> Self {
        Self {
            methods: Vec::new(),
            reserve: PyPaymentReserve::new(),
        }
    }
}

impl PyPayment {
    pub fn to_rust_payment(&self) -> PyResult<Payment> {
        let mut payment_methods = Vec::new();
        for method in &self.methods {
            payment_methods.push(method.to_rust_method()?);
        }
        Ok(Payment {
            methods: payment_methods,
            reserve: self.reserve.to_rust_reserve(),
        })
    }

    pub fn from_rust_payment(payment: &Payment) -> Self {
        Self {
            methods: payment.methods.iter()
                .map(|m| PyPaymentMethod::from_rust_method(m))
                .collect(),
            reserve: PyPaymentReserve::from_rust_reserve(&payment.reserve),
        }
    }
}

/// Python-compatible PaymentMethod
#[pyclass]
#[derive(Clone)]
pub struct PyPaymentMethod {
    #[pyo3(get, set)]
    pub method_type: String,
    #[pyo3(get, set)]
    pub amount: u32,
}

#[pymethods]
impl PyPaymentMethod {
    #[new]
    fn new(method_type: String, amount: u32) -> Self {
        Self { method_type, amount }
    }
}

impl PyPaymentMethod {
    pub fn to_rust_method(&self) -> PyResult<PaymentMethod> {
        match self.method_type.as_str() {
            "MegaCredits" => Ok(PaymentMethod::MegaCredits(self.amount)),
            "Steel" => Ok(PaymentMethod::Steel(self.amount)),
            "Titanium" => Ok(PaymentMethod::Titanium(self.amount)),
            "Heat" => Ok(PaymentMethod::Heat(self.amount)),
            "Plants" => Ok(PaymentMethod::Plants(self.amount)),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Unknown payment method type: {}", self.method_type)
            )),
        }
    }

    pub fn from_rust_method(method: &PaymentMethod) -> Self {
        match method {
            PaymentMethod::MegaCredits(amount) => Self {
                method_type: "MegaCredits".to_string(),
                amount: *amount,
            },
            PaymentMethod::Steel(amount) => Self {
                method_type: "Steel".to_string(),
                amount: *amount,
            },
            PaymentMethod::Titanium(amount) => Self {
                method_type: "Titanium".to_string(),
                amount: *amount,
            },
            PaymentMethod::Heat(amount) => Self {
                method_type: "Heat".to_string(),
                amount: *amount,
            },
            PaymentMethod::Plants(amount) => Self {
                method_type: "Plants".to_string(),
                amount: *amount,
            },
        }
    }
}

/// Python-compatible PaymentReserve
#[pyclass]
#[derive(Clone)]
pub struct PyPaymentReserve {
    #[pyo3(get, set)]
    pub megacredits: u32,
    #[pyo3(get, set)]
    pub steel: u32,
    #[pyo3(get, set)]
    pub titanium: u32,
    #[pyo3(get, set)]
    pub heat: u32,
    #[pyo3(get, set)]
    pub plants: u32,
}

#[pymethods]
impl PyPaymentReserve {
    #[new]
    fn new() -> Self {
        Self {
            megacredits: 0,
            steel: 0,
            titanium: 0,
            heat: 0,
            plants: 0,
        }
    }
}

impl PyPaymentReserve {
    pub fn to_rust_reserve(&self) -> PaymentReserve {
        PaymentReserve {
            megacredits: self.megacredits,
            steel: self.steel,
            titanium: self.titanium,
            heat: self.heat,
            plants: self.plants,
        }
    }

    pub fn from_rust_reserve(reserve: &PaymentReserve) -> Self {
        Self {
            megacredits: reserve.megacredits,
            steel: reserve.steel,
            titanium: reserve.titanium,
            heat: reserve.heat,
            plants: reserve.plants,
        }
    }
}

/// Python-compatible StandardProjectParams
#[pyclass]
#[derive(Clone)]
pub struct PyStandardProjectParams {
    #[pyo3(get, set)]
    pub card_ids: Vec<String>,
}

#[pymethods]
impl PyStandardProjectParams {
    #[new]
    fn new() -> Self {
        Self {
            card_ids: Vec::new(),
        }
    }
}

impl PyStandardProjectParams {
    pub fn to_rust_params(&self) -> StandardProjectParams {
        StandardProjectParams {
            card_ids: self.card_ids.clone(),
        }
    }

    pub fn from_rust_params(params: &StandardProjectParams) -> Self {
        Self {
            card_ids: params.card_ids.clone(),
        }
    }
}

/// Python-compatible Phase enum
#[pyclass]
#[derive(Clone)]
pub struct PyPhase {
    #[pyo3(get, set)]
    pub phase: String,
}

#[pymethods]
impl PyPhase {
    #[new]
    fn new(phase: String) -> Self {
        Self { phase }
    }
}

impl PyPhase {
    pub fn from_rust_phase(phase: &Phase) -> Self {
        Self {
            phase: format!("{:?}", phase),
        }
    }
}

