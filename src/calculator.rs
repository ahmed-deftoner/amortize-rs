use std::fmt;
use chrono::NaiveDate;
use crate::payment::Payment;
use crate::error::AmortizationError;

#[derive(Debug, Clone)]
pub struct Amortization {
    pub balance: f64,            
    pub periods: u32,            
    pub periodic_interest: f64,  
    pub periodic_payment: f64,   
    pub schedule: Vec<Payment>,   
    pub total_payment: f64,       
    pub total_interest: f64,      
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,  
}

impl fmt::Display for Amortization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Amortization:")?;
        writeln!(f, "Loan Amount: {:.2}", self.balance)?;
        writeln!(f, "Periodic Interest Rate: {:.4}", self.periodic_interest)?;
        writeln!(f, "Total Periods: {}", self.periods)?;
        writeln!(f, "Periodic Payment: {:.2}", self.periodic_payment)?;
        writeln!(f, "Total Payment: {:.2}", self.total_payment)?;
        writeln!(f, "Total Interest: {:.2}", self.total_interest)?;
        writeln!(f, "Amortization Schedule:")?;

        for (i, payment) in self.schedule.iter().enumerate() {
            writeln!(f, "Payment {}: {}", i + 1, payment)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CalculatorConfig {
    pub balance: f64,          
    pub loan_term: u32,        
    pub apr: f64,              
    pub start_date: Option<NaiveDate>,
}

impl Amortization {
    pub fn new(balance: f64, apr: f64, periods: u32, start_date: Option<NaiveDate>) -> Result<Self, AmortizationError>  {
        if periods == 0 {
            return Err(AmortizationError::InvalidPeriods(periods));
        }
        if apr <= 0.0 {
            return Err(AmortizationError::InvalidInterestRate(apr));
        }
        if balance <= 0.0 {
            return Err(AmortizationError::InvalidLoanAmount(balance));
        }
        let periodic_interest = apr / 100.0 / 12.0; 

        let mut amortization = Amortization {
            balance,
            periods,
            periodic_interest,
            periodic_payment: 0.0, 
            schedule: Vec::new(),  
            total_payment: 0.0,    
            total_interest: 0.0,   
            start_date,
            end_date: start_date,  
        };

        amortization.periodic_payment = amortization.calculate_periodic_payment_amount()?;
        amortization.schedule = amortization.calculate_schedule()?;
        amortization.total_payment = amortization.calculate_total_payment();
        amortization.total_interest = amortization.calculate_total_interest();

        Ok(amortization)
    }

    pub fn calculate_periodic_payment_amount(&self) -> Result<f64, AmortizationError> {
        let rate = self.periodic_interest;
        let nper = self.periods as f64;
        let pv = self.balance;

        let base = 1.0 + rate;
        let exp = base.powf(nper);
        
        if exp.is_infinite() || exp.is_nan() {
            return Err(AmortizationError::CalculationError(
                "Overflow in payment calculation".to_string()
            ));
        }

        // Using the PMT formula: PMT = PV * (r * (1 + r)^n) / ((1 + r)^n - 1)
        let payment = pv * (rate * exp) / (exp - 1.0);

        if payment.is_infinite() || payment.is_nan() {
            return Err(AmortizationError::CalculationError(
                "Invalid payment calculation result".to_string()
            ));
        }
        
        Ok((payment * 100.0).round() / 100.0)
    }
    
    pub fn calculate_total_payment(&self) -> f64 {
        self.periods as f64 * self.periodic_payment
    }

    pub fn calculate_total_interest(&self) -> f64 {
        self.total_payment - self.balance
    }

    pub fn calculate_payment(&self, balance: f64, installment_number: u32, beginning_balance: f64) ->  Result<Payment, AmortizationError> {
        let interest = balance * self.periodic_interest;

        if interest.is_nan() || interest.is_infinite() {
            return Err(AmortizationError::CalculationError(
                "Invalid interest calculation".to_string()
            ));
        }

        let principal = if balance < self.periodic_payment {
            balance
        } else {
            self.periodic_payment - interest
        };

        if principal.is_nan() || principal.is_infinite() {
            return Err(AmortizationError::CalculationError(
                "Invalid principal calculation".to_string()
            ));
        }

        let remaining_balance = if balance < self.periodic_payment {
            0.0 
        } else {
            balance - principal
        };
        let ending_balance = beginning_balance - principal;

        Ok(Payment {
            installment_number,
            beginning_balance,
            ending_balance,
            installment_amount: self.periodic_payment,
            interest,
            principal,
            remaining_balance,
            date: None, 
        })
    }

    pub fn calculate_schedule(&mut self) -> Result<Vec<Payment>, AmortizationError> {
        let mut balance = self.balance;
        let mut schedule = Vec::new();
        let mut current_date = self.start_date;
        let mut installment_number = 1;
        let mut beginning_balance = self.balance;
        
        while balance > 0.0 {
            let mut payment = self.calculate_payment(balance, installment_number, beginning_balance)?;
            balance = payment.remaining_balance;
            installment_number += 1;

            if let Some(ref mut end_date) = current_date {
                payment.date = Some(*end_date);
                *end_date = end_date.checked_add_months(chrono::Months::new(1))
                    .ok_or_else(|| AmortizationError::CalculationError(
                        "Invalid date calculation".to_string()
                    ))?;
            }

            schedule.push(payment.clone());

            beginning_balance = beginning_balance - payment.principal;
        }

        self.end_date = current_date;
        Ok(schedule)
    }
}