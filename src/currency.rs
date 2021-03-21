use std::{
    fmt,
    ops::{Add, AddAssign, Neg, SubAssign},
    str::FromStr,
};

#[derive(Debug)]
pub struct ParseCurrencyError;
/// Datatype for the currency used in the csv, as we atmost have 4 decimals of precision
/// then a i64 should be plenty to hold the values.
/// The current implementation allows amounts of up to 2^63 / 1000 or around 300 trillion with 4 decimal precision
/// this is more than 30 times the entire worlds wealth
/// Alternative approach is using either rust_decimal and some BigNumber lib, but that would hurt the performance quite a bit
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Currency(i64);

impl Currency {
    #[allow(dead_code)]
    pub fn new(x: i64) -> Self {
        Self(x)
    }
}

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split('.');
        let first = splitted.next().map(i64::from_str);
        let second = splitted
            .next()
            .map(|s| format!("{:0<4}", s))
            .map(|s| i64::from_str(&s));
        match (first, second) {
            (Some(Ok(first)), None) => Ok(Currency(first * 10000)),
            (Some(Ok(first)), Some(Ok(second))) => {
                let first = first * 10000;
                let second = if first.is_negative() { -second } else { second };

                Ok(Currency(first + second))
            }
            _ => Err(ParseCurrencyError),
        }
    }
}

impl Add for Currency {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0)
    }
}

impl AddAssign for Currency {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}

impl SubAssign for Currency {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}

impl Neg for Currency {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Currency(-self.0)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:0>4}", self.0 / 10000, self.0.abs() % 10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_positive_strings() {
        let num1 = "1.5";
        let num2 = "1.50";
        let num3 = "1.500";
        let num4 = "1.5000";
        assert_eq!(Currency::from_str(num1).unwrap(), Currency(15000));
        assert_eq!(Currency::from_str(num2).unwrap(), Currency(15000));
        assert_eq!(Currency::from_str(num3).unwrap(), Currency(15000));
        assert_eq!(Currency::from_str(num4).unwrap(), Currency(15000));
    }

    #[test]
    fn can_parse_negative_strings() {
        let num1 = "-1.5";
        let num2 = "-1.50";
        let num3 = "-1.500";
        let num4 = "-1.5000";
        assert_eq!(Currency::from_str(num1).unwrap(), Currency(-15000));
        assert_eq!(Currency::from_str(num2).unwrap(), Currency(-15000));
        assert_eq!(Currency::from_str(num3).unwrap(), Currency(-15000));
        assert_eq!(Currency::from_str(num4).unwrap(), Currency(-15000));
    }

    #[test]
    fn can_parse_all_decimals() {
        let num1 = "1.0005";
        let num2 = "1.0050";
        let num3 = "1.0500";
        let num4 = "1.5000";
        assert_eq!(Currency::from_str(num1).unwrap(), Currency(10005));
        assert_eq!(Currency::from_str(num2).unwrap(), Currency(10050));
        assert_eq!(Currency::from_str(num3).unwrap(), Currency(10500));
        assert_eq!(Currency::from_str(num4).unwrap(), Currency(15000));
    }

    #[test]
    fn can_convert_to_string() {
        let pos_currency1 = Currency(15000);
        let neg_currency1 = Currency(-15000);
        let pos_currency2 = Currency(10500);
        let neg_currency2 = Currency(-10500);
        let pos_currency3 = Currency(10050);
        let neg_currency3 = Currency(-10050);
        let pos_currency4 = Currency(10005);
        let neg_currency4 = Currency(-10005);
        assert_eq!(pos_currency1.to_string(), "1.5000");
        assert_eq!(neg_currency1.to_string(), "-1.5000");
        assert_eq!(pos_currency2.to_string(), "1.0500");
        assert_eq!(neg_currency2.to_string(), "-1.0500");
        assert_eq!(pos_currency3.to_string(), "1.0050");
        assert_eq!(neg_currency3.to_string(), "-1.0050");
        assert_eq!(pos_currency4.to_string(), "1.0005");
        assert_eq!(neg_currency4.to_string(), "-1.0005");
    }

    #[test]
    fn negation() {
        let pos_currency = Currency(15000);
        let neg_currency = Currency(-15000);
        assert_eq!(-pos_currency, neg_currency);
        assert_eq!(-neg_currency, pos_currency);
    }

    #[test]
    fn addition() {
        let num0 = Currency(0);
        let num1 = Currency(15000);
        let num2 = Currency(-15000);
        let num3 = Currency(30000);
        assert_eq!(num1 + num2, num0);
        assert_eq!(num1 + num1, num3);
        assert_eq!(num3 + num2, num1);
    }

    #[test]
    fn add_assign() {
        let mut num0 = Currency(0);
        let num1 = Currency(15000);
        let num2 = Currency(-15000);
        num0 += num1;
        assert_eq!(num0, num1);
        num0 += num2;
        assert_eq!(num0, Currency(0));
    }

    #[test]
    fn sub_assign() {
        let num1 = Currency(15000);
        let num2 = Currency(-15000);
        let mut num3 = Currency(30000);
        num3 -= num1;
        assert_eq!(num3, num1);
        num3 -= num2;
        assert_eq!(num3, Currency(30000));
    }
}
