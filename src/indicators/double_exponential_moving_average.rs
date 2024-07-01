use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
use crate::indicators::ExponentialMovingAverage;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An exponential moving average (EMA), also known as an exponentially weighted moving average
/// (EWMA).
///
/// It is a type of infinite impulse response filter that applies weighting factors which decrease exponentially.
/// The weighting for each older datum decreases exponentially, never reaching zero.
///
/// # Formula
///
/// ![EMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/05d06bdbee2c14031fd91ead6f5f772aec1ec964)
///
/// Where:
///
/// * _EMA<sub>t</sub>_ - is the value of the EMA at any time period _t_.
/// * _EMA<sub>t-1</sub>_ - is the value of the EMA at the previous period _t-1_.
/// * _p<sub>t</sub>_ - is the input value at a time period t.
/// * _α_ - is the coefficient that represents the degree of weighting decrease, a constant smoothing factor between 0 and 1.
///
/// _α_ is calculated with the following formula:
///
/// ![alpha formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/d9f6258e152db0644af548972bd6c50a8becf7ee)
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
/// use ta::indicators::DoubleExponentialMovingAverage;
/// use ta::Next;
///
/// let mut ema = DoubleExponentialMovingAverage::new(3).unwrap();
/// assert_eq!(ema.next(2.0), 2.0);
/// assert_eq!(ema.next(5.0), 4.25);
/// assert_eq!(ema.next(1.0), 2.0);
/// assert_eq!(ema.next(6.25), 5.125);
/// ```
///
/// # Links
///
/// * [Exponential moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average)
///



#[doc(alias = "DEMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct DoubleExponentialMovingAverage {
    period: usize,
    // k: f64,
    current: f64,
    is_new: bool,
    ema: ExponentialMovingAverage,
    ema2: ExponentialMovingAverage,
}

impl DoubleExponentialMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                // k: 2.0 / (period + 1) as f64,
                current: 0.0,
                is_new: true,
                ema: ExponentialMovingAverage::new(period).unwrap(),
                ema2: ExponentialMovingAverage::new(period).unwrap(),
                
            }),
        }
    }
}

impl Period for DoubleExponentialMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for DoubleExponentialMovingAverage {
    type Output = f64;

    
    fn next(&mut self, input: f64) -> Self::Output {

        let ema_value = self.ema.next(input);

        if self.is_new {
            self.is_new = false;
            self.current = self.ema2.next(ema_value);
        } else {

            let ema_2_value = self.ema2.next(ema_value); 

            self.current = (2.0 * ema_value) - ema_2_value;
        }

        self.current
    }
}

impl<T: Close> Next<&T> for DoubleExponentialMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for DoubleExponentialMovingAverage {
    fn reset(&mut self) {
        self.current = 0.0;
        self.is_new = true;

        self.ema.reset();
        self.ema2.reset();
    }
}

impl Default for DoubleExponentialMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for DoubleExponentialMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(DoubleExponentialMovingAverage);

    #[test]
    fn test_new() {
        assert!(DoubleExponentialMovingAverage::new(0).is_err());
        assert!(DoubleExponentialMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut ema = DoubleExponentialMovingAverage::new(3).unwrap();

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 4.25);
        assert_eq!(ema.next(1.0), 2.0);
        assert_eq!(ema.next(6.25), 5.125);

        let mut ema = DoubleExponentialMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(ema.next(&bar1), 2.0);
        assert_eq!(ema.next(&bar2), 4.25);
    }

    #[test]
    fn test_reset() {
        let mut ema = DoubleExponentialMovingAverage::new(5).unwrap();

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        DoubleExponentialMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let ema = DoubleExponentialMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", ema), "DEMA(7)");
    }
}
