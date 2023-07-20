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
