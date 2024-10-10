pub mod calculator;
pub mod payment;

pub use calculator::Amortization;
pub use payment::Payment;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::f64::EPSILON;

    const FLOAT_PRECISION: f64 = 0.01;

    fn assert_float_eq(a: f64, b: f64) {
        assert!((a - b).abs() < FLOAT_PRECISION, "Expected {}, got {}", b, a);
    }

    #[test]
    fn test_new_amortization() {
        let loan = Amortization::new(
            100_000.0,
            5.0,
            360,
            None
        );

        assert_float_eq(loan.balance, 100_000.0);
        assert_float_eq(loan.periodic_interest, 0.05 / 12.0);
        assert_eq!(loan.periods, 360);
    }

    #[test]
    fn test_monthly_payment_calculation() {
        // Test case 1: 30-year mortgage
        let loan1 = Amortization::new(200_000.0, 3.5, 360, None);
        assert_float_eq(loan1.periodic_payment, 898.09);

        // Test case 2: 5-year loan
        let loan2 = Amortization::new(280_350.0, 3.5, 60, None);
        assert_float_eq(loan2.periodic_payment, 5_112.40);

        // Test case 3: Small loan
        let loan3 = Amortization::new(10_000.0, 5.0, 12, None);
        assert_float_eq(loan3.periodic_payment, 856.07);
    }

    #[test]
    fn test_total_payment_calculation() {
        let loan = Amortization::new(100_000.0, 5.0, 360, None);
        let expected_total = loan.periodic_payment * 360.0;
        assert_float_eq(loan.total_payment, expected_total);
    }

    #[test]
    fn test_total_interest_calculation() {
        let loan = Amortization::new(100_000.0, 5.0, 360, None);
        let expected_interest = loan.total_payment - loan.balance;
        assert_float_eq(loan.total_interest, expected_interest);
    }

    #[test]
    fn test_payment_schedule_generation() {
        let loan = Amortization::new(
            10_000.0,
            5.0,
            12,
            Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        );

        assert_eq!(loan.schedule.len(), 12);
        
        // Test first payment
        let first_payment = &loan.schedule[0];
        assert_eq!(first_payment.installment_number, 1);
        assert_float_eq(first_payment.beginning_balance, 10_000.0);
        assert!(first_payment.interest > 0.0);
        assert!(first_payment.principal > 0.0);
        assert_float_eq(
            first_payment.interest + first_payment.principal,
            loan.periodic_payment
        );

        // Test last payment
        let last_payment = &loan.schedule[11];
        assert_float_eq(last_payment.remaining_balance, 0.0);
    }

    #[test]
    fn test_payment_dates() {
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let loan = Amortization::new(10_000.0, 5.0, 12, Some(start_date));

        // Check first payment date
        assert_eq!(loan.schedule[0].date.unwrap(), start_date);

        // Check last payment date
        let expected_end_date = start_date + chrono::Duration::days(30 * 11);
        assert_eq!(loan.schedule[11].date.unwrap(), expected_end_date);
    }

    #[test]
    fn test_edge_cases() {
        // Test very small loan
        let small_loan = Amortization::new(100.0, 5.0, 12, None);
        assert!(small_loan.periodic_payment > 0.0);
        assert_eq!(small_loan.schedule.len(), 12);

        // Test very large loan
        let large_loan = Amortization::new(1_000_000.0, 3.5, 360, None);
        assert!(large_loan.periodic_payment > 0.0);
        assert_eq!(large_loan.schedule.len(), 360);

        // Test short term
        let short_term = Amortization::new(10_000.0, 5.0, 3, None);
        assert_eq!(short_term.schedule.len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_periods() {
        Amortization::new(10_000.0, 5.0, 0, None);
    }

    #[test]
    #[should_panic]
    fn test_invalid_interest_rate() {
        Amortization::new(10_000.0, -5.0, 12, None);
    }
}