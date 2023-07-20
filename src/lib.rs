//! Stirling's Approximation
//!
//! Provide 2 functions (to_f64 and to_bigdecimal) that calculates the
//! factorial of a number using the Stirling's Approximation formula.
//!
//! This formula let you calculate the factorial with a very good precision
//! for big numbers, and is not recursive, which saves a lot of computation
//! time.
//!
//! The formula is the following:
//! n! = sqrt(2 * PI * n) * (n / E) ^ n
//!
//! # Example
//!
//! ```
//! use stirling_approximation;
//!
//! let factorial = stirling_approximation::to_f64(10);
//! let high_precision_factorial = stirling_approximation::to_bigdecimal(10);
//!
//! println!("The factorial of 10 is: {}", factorial);
//! println!("The high precision factorial of 10 is: {}", high_precision_factorial);
//! ```
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::{
    f64::consts::{E, PI},
    ops::Mul,
};

fn bigdecimal_power(base: &BigDecimal, exponent: u32) -> BigDecimal {
    let mut current: BigDecimal = base.clone();
    for _ in 1..exponent {
        current = current.mul(base);
    }
    current
}

pub fn to_f64(n: i32) -> f64 {
    (2.0 * PI * n as f64).sqrt() * ((n as f64) / E).powi(n)
}

pub fn to_bigdecimal(n: i32) -> Option<BigDecimal> {
    let n = BigDecimal::from_i32(n)?;
    Some(
        bigdecimal_power(&(&n / E), n.to_u32()?)
            .mul(BigDecimal::from_f64((2.0 * PI * n.to_f64()?).sqrt())?),
    )
}

#[cfg(test)]
mod tests {}
