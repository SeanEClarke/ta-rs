use std::fmt;

use crate::errors::{Result, TaError};
use crate::indicators::ExponentialMovingAverage;
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A triple exponential moving average (TRIX)
///
/// It is a type of infinite impulse response filter that applies weighting factors which decrease exponentially.
/// The weighting for each older datum decreases exponentially, never reaching zero.
///
/// Where:
///
/// * _period_ - number of periods
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::TripleExponentialAverage;
/// use ta::Next;
///
/// let mut trix = TripleExponentialAverage::new(3).unwrap();
/// let result = trix.next(2.0);
/// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Triple_exponential_moving_average)
///

#[doc(alias = "TRIX")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct TripleExponentialAverage {
    period: usize,
    // k: f64,
    current_em_3_value: f64,
    is_new: bool,
    ema: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
    ema3: ExponentialMovingAverage,
}

impl TripleExponentialAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                // k: 2.0 / (period + 1) as f64,
                current_em_3_value: 0.0,
                is_new: true,
                ema: ExponentialMovingAverage::new(period).unwrap(),
                ema2: ExponentialMovingAverage::new(period).unwrap(),
                ema3: ExponentialMovingAverage::new(period).unwrap(),
            }),
        }
    }
}

impl Period for TripleExponentialAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for TripleExponentialAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let ema_value = self.ema.next(input);
        let ema_2_value = self.ema2.next(ema_value);
        let ema_3_value = self.ema3.next(ema_2_value);

        let mut trix = 0.0;

        if self.is_new {
            self.is_new = false;
            self.current_em_3_value = ema_3_value;
        } else {
            trix = ((ema_3_value - self.current_em_3_value) / self.current_em_3_value) * 100.0;
            self.current_em_3_value = ema_3_value;
        }

        trix
    }
}

impl<T: Close> Next<&T> for TripleExponentialAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for TripleExponentialAverage {
    fn reset(&mut self) {
        self.current_em_3_value = 0.0;
        self.is_new = true;

        self.ema.reset();
        self.ema2.reset();
        self.ema3.reset();
    }
}

impl Default for TripleExponentialAverage {
    fn default() -> Self {
        Self::new(15).unwrap()
    }
}

impl fmt::Display for TripleExponentialAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRIX({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(TripleExponentialAverage);

    #[test]
    fn test_new() {
        assert!(TripleExponentialAverage::new(0).is_err());
        assert!(TripleExponentialAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut trix = TripleExponentialAverage::new(3).unwrap();

        assert_eq!(trix.next(16.0), 0.0);
        assert_eq!(trix.next(17.0), 0.78125);
        assert_eq!(trix.next(17.0), 1.1627906976744187);
        assert_eq!(trix.next(10.0), -4.21455938697318);
        assert_eq!(trix.next(17.0), -1.7999999999999998);

        let mut trix = TripleExponentialAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(trix.next(&bar1), 0.0);
        assert_eq!(trix.next(&bar2), 18.75);
    }

    #[test]
    fn test_next_2() {
        let mut trix = TripleExponentialAverage::new(15).unwrap();

        trix.next(16.0);
        trix.next(17.0);
        trix.next(17.0);
        trix.next(10.0);
        trix.next(17.0);
        trix.next(18.0);
        trix.next(17.0);
        trix.next(17.0);
        let result = trix.next(17.0);

        assert_eq!(result, 0.029258774080521098);
    }

    #[test]
    fn test_reset() {
        let mut trix = TripleExponentialAverage::new(5).unwrap();

        assert_eq!(trix.next(4.0), 0.0);
        trix.next(10.0);
        trix.next(15.0);
        trix.next(20.0);
        assert_ne!(trix.next(4.0), 4.0);

        trix.reset();
        assert_eq!(trix.next(4.0), 0.0);
    }

    #[test]
    fn test_default() {
        TripleExponentialAverage::default();
    }

    #[test]
    fn test_display() {
        let trix = TripleExponentialAverage::new(7).unwrap();
        assert_eq!(format!("{}", trix), "TRIX(7)");
    }
}
