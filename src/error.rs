use std::fmt;

#[derive(Debug)]
pub enum AmortizationError {
    InvalidPeriods(u32),
    InvalidInterestRate(f64),
    InvalidLoanAmount(f64),
    CalculationError(String),
}

impl std::error::Error for AmortizationError {}

impl fmt::Display for AmortizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AmortizationError::InvalidPeriods(p) => write!(f, "Number of periods must be greater than 0, got {}", p),
            AmortizationError::InvalidInterestRate(r) => write!(f, "Interest rate must be greater than 0, got {}", r),
            AmortizationError::InvalidLoanAmount(a) => write!(f, "Loan amount must be greater than 0, got {}", a),
            AmortizationError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}
