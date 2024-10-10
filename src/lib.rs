pub mod calculator;
pub mod payment;
pub mod error;

#[cfg(test)]
mod tests;

pub use calculator::Amortization;
pub use payment::Payment;
pub use error::AmortizationError;