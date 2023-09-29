use color_eyre::eyre::Error;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

const DECIMALS: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Decimal(i64);

impl Decimal {
    pub fn int(val: i64) -> Self {
        Self(val * 10i64.pow(DECIMALS as u32))
    }
}

pub fn dec(val: &str) -> Decimal {
    Decimal::from_str(val).unwrap()
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let integer = parts.next().unwrap_or("0");
        let fractional = parts.next().unwrap_or("0");
        let fractional = format!("{:0<2}", fractional);
        let value = format!("{}{}", integer, fractional);
        let value = value.parse::<i64>()?;
        Ok(Self(value))
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as f64 / 10i64.pow(DECIMALS as u32) as f64)
    }
}

impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Decimal> for i64 {
    fn from(val: Decimal) -> Self {
        val.0
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Decimal {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sub for &Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0 - rhs.0)
    }
}

impl SubAssign for Decimal {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0 / 10i64.pow(DECIMALS as u32))
    }
}

impl MulAssign for Decimal {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0 / 10i64.pow(DECIMALS as u32);
    }
}

impl Div for Decimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 * 10i64.pow(DECIMALS as u32) / rhs.0)
    }
}

impl DivAssign for Decimal {
    fn div_assign(&mut self, rhs: Self) {
        self.0 *= 10i64.pow(DECIMALS as u32) / rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance() {
        assert_eq!(Decimal(0).to_string(), "0");
        assert_eq!(Decimal(1).to_string(), "0.01");
        assert_eq!(Decimal(10).to_string(), "0.1");
        assert_eq!(Decimal(101).to_string(), "1.01");
        assert_eq!(Decimal(1010).to_string(), "10.1");
        assert_eq!(Decimal(10101).to_string(), "101.01");
        assert_eq!(Decimal(101010).to_string(), "1010.1");
        assert_eq!(Decimal(1010101).to_string(), "10101.01");
    }

    #[test]
    fn test_balance_from_str() {
        assert_eq!(Decimal::from_str("0").unwrap().0, 0);
        assert_eq!(Decimal::from_str("1000").unwrap().0, 100000);
        assert_eq!(Decimal::from_str("0.01").unwrap().0, 1);
        assert_eq!(Decimal::from_str("0.1").unwrap().0, 10);
        assert_eq!(Decimal::from_str("1.01").unwrap().0, 101);
        assert_eq!(Decimal::from_str("10.1").unwrap().0, 1010);
        assert_eq!(Decimal::from_str("101.01").unwrap().0, 10101);
        assert_eq!(Decimal::from_str("1010.1").unwrap().0, 101010);
        assert_eq!(Decimal::from_str("10101.01").unwrap().0, 1010101);
    }

    #[test]
    fn test_balance_add() {
        assert_eq!(Decimal(0) + Decimal(0), Decimal(0));
        assert_eq!(Decimal(0) + Decimal(1), Decimal(1));
        assert_eq!(Decimal(1) + Decimal(0), Decimal(1));
        assert_eq!(Decimal(1) + Decimal(1), Decimal(2));
        assert_eq!(Decimal(1) + Decimal(10), Decimal(11));
        assert_eq!(Decimal(10) + Decimal(1), Decimal(11));
        assert_eq!(Decimal(10) + Decimal(10), Decimal(20));
        assert_eq!(Decimal(101) + Decimal(101), Decimal(202));
        assert_eq!(Decimal(1010) + Decimal(1010), Decimal(2020));
        assert_eq!(Decimal(10101) + Decimal(10101), Decimal(20202));
        assert_eq!(Decimal(101010) + Decimal(101010), Decimal(202020));
        assert_eq!(Decimal(1010101) + Decimal(1010101), Decimal(2020202));
    }

    #[test]
    fn test_balance_add_assign() {
        let mut balance = Decimal(0);
        balance += Decimal(0);
        assert_eq!(balance, Decimal(0));
        balance += Decimal(1);
        assert_eq!(balance, Decimal(1));
        balance += Decimal(10);
        assert_eq!(balance, Decimal(11));
        balance += Decimal(101);
        assert_eq!(balance, Decimal(112));
        balance += Decimal(1010);
        assert_eq!(balance, Decimal(1122));
        balance += Decimal(10101);
        assert_eq!(balance, Decimal(11223));
        balance += Decimal(101010);
        assert_eq!(balance, Decimal(112234));
        balance += Decimal(1010101);
        assert_eq!(balance, Decimal(1122343));
    }

    #[test]
    fn test_balance_sub() {
        assert_eq!(Decimal(0) - Decimal(0), Decimal(0));
        assert_eq!(Decimal(1) - Decimal(0), Decimal(1));
        assert_eq!(Decimal(1) - Decimal(1), Decimal(0));
        assert_eq!(Decimal(1) - Decimal(10), Decimal(-9));
        assert_eq!(Decimal(10) - Decimal(1), Decimal(9));
        assert_eq!(Decimal(10) - Decimal(10), Decimal(0));
        assert_eq!(Decimal(101) - Decimal(101), Decimal(0));
        assert_eq!(Decimal(1010) - Decimal(1010), Decimal(0));
        assert_eq!(Decimal(10101) - Decimal(10101), Decimal(0));
        assert_eq!(Decimal(101010) - Decimal(101010), Decimal(0));
        assert_eq!(Decimal(1010101) - Decimal(1010101), Decimal(0));
    }

    #[test]
    fn test_balance_sub_assign() {
        let mut balance = Decimal(0);
        balance -= Decimal(0);
        assert_eq!(balance, Decimal(0));
        balance -= Decimal(1);
        assert_eq!(balance, Decimal(-1));
        balance -= Decimal(10);
        assert_eq!(balance, Decimal(-11));
        balance -= Decimal(101);
        assert_eq!(balance, Decimal(-112));
        balance -= Decimal(1010);
        assert_eq!(balance, Decimal(-1122));
        balance -= Decimal(10101);
        assert_eq!(balance, Decimal(-11223));
    }

    #[test]
    fn test_balance_mul() {
        assert_eq!(Decimal(0) * Decimal(0), Decimal(0));
        assert_eq!(Decimal(0) * Decimal(1), Decimal(0));
        assert_eq!(Decimal(1) * Decimal(0), Decimal(0));
        assert_eq!(Decimal(1) * Decimal(1), Decimal(0));
        assert_eq!(Decimal(1) * Decimal(10), Decimal(0));
        assert_eq!(Decimal(10) * Decimal(1), Decimal(0));
        assert_eq!(
            Decimal::from_str("0.1").unwrap() * Decimal::from_str("0.1").unwrap(),
            Decimal::from_str("0.01").unwrap()
        );
        assert_eq!(
            Decimal::from_str("0.1").unwrap() * Decimal::from_str("0.01").unwrap(),
            Decimal::from_str("0.0").unwrap()
        );
        assert_eq!(
            Decimal::from_str("1.1").unwrap() * Decimal::from_str("5.01").unwrap(),
            Decimal::from_str("5.51").unwrap()
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Decimal::from_str("10").unwrap() / Decimal::from_str("2").unwrap(),
            Decimal::from_str("5").unwrap()
        );
        assert_eq!(
            Decimal::from_str("10").unwrap() / Decimal::from_str("3").unwrap(),
            Decimal::from_str("3.33").unwrap()
        );
    }
}
