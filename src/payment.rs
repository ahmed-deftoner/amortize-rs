use std::fmt;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Payment {
    pub installment_number: u32,
    pub beginning_balance: f64,
    pub ending_balance: f64,
    pub interest: f64,
    pub principal: f64,
    pub date: Option<NaiveDate>,
    pub remaining_balance: f64,
    pub installment_amount: f64
}

impl fmt::Display for Payment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Date: {:?}, Interest: {:.2}, Principal: {:.2}, Remaining Balance: {:.2}, Beginning Balance: {:.2} Ending Balance: {:.2}",
            self.date.unwrap(), self.interest, self.principal, self.remaining_balance, self.beginning_balance, self.ending_balance
        )
    }
}