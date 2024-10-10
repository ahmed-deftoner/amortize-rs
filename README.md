# Amortization Calculator

A Rust library for calculating loan amortization schedules.

## Features
- Calculate monthly payment amounts
- Generate complete amortization schedules
- Track principal and interest payments

## Usage

```rust
use amortization_calculator::Amortization;
use chrono::NaiveDate;

fn main() {
    let loan = Amortization::new(
        280350.0, // Principal
        3.5,      // APR
        60,       // Periods (5 years)
        Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
    );

    println!("{}", loan);
}
```