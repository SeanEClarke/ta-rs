use std::fmt;

use crate::{Close, High, Low, Next, Reset, Volume};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Volume Weighted Average Price (VWAP).
///
///
/// # Links
///
/// * [On Balance Volume, Wikipedia](https://en.wikipedia.org/wiki/Volume-weighted_average_price)

#[doc(alias = "OBV")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct VolumeWeightedAveragePrice {
    accumulated_price_volume: f64,
    accumulated_volume: f64,
}

impl VolumeWeightedAveragePrice {
    pub fn new() -> Self {
        Self {
            accumulated_price_volume: 0.0,
            accumulated_volume: 0.0,
        }
    }
}

impl<T: High + Low + Close + Volume> Next<&T> for VolumeWeightedAveragePrice {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        let pv = ((input.high() + input.low() + input.close()) / 3.0) * input.volume();

        self.accumulated_price_volume += pv;
        self.accumulated_volume += input.volume();

        if self.accumulated_volume.abs() < 0.0001 {
            return self.accumulated_price_volume;
        }

        self.accumulated_price_volume / self.accumulated_volume
    }
}

impl Default for VolumeWeightedAveragePrice {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for VolumeWeightedAveragePrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OBV")
    }
}

impl Reset for VolumeWeightedAveragePrice {
    fn reset(&mut self) {
        self.accumulated_price_volume = 0.0;
        self.accumulated_volume = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    #[test]
    fn test_next_bar() {
        let mut vwap = VolumeWeightedAveragePrice::new();

        let bar = Bar::new().high(1.3).low(0.8).close(1.1).volume(100.0);

        let result = vwap.next(&bar);

        assert_eq!(result, (1.3 + 0.8 + 1.1) / 3.0);
    }

    #[test]
    fn test_next_bar_2() {
        let mut vwap = VolumeWeightedAveragePrice::new();

        {
            let bar = Bar::new().high(1.3).low(0.8).close(1.1).volume(100.0);
            vwap.next(&bar);
        }

        {
            let bar = Bar::new().high(1.4).low(1.0).close(1.3).volume(250.0);

            vwap.next(&bar);
        }

        let bar = Bar::new().high(1.6).low(1.3).close(1.5).volume(150.0);

        let result = vwap.next(&bar);

        println!("{}", result);
        println!("{}", (result - 1.27).abs());
        println!("{:?}", (result - 1.27).abs() < f64::EPSILON);

        assert!((result - 1.27).abs() < 0.0001);
    }

    #[test]
    fn test_reset() {
        let mut vwap = VolumeWeightedAveragePrice::new();

        let bar = Bar::new().high(1.3).low(0.8).close(1.1).volume(100.0);
        let result = vwap.next(&bar);
        assert_eq!(result, (1.3 + 0.8 + 1.1) / 3.0);

        vwap.reset();

        let result = vwap.next(&bar);
        assert_eq!(result, (1.3 + 0.8 + 1.1) / 3.0);
    }

    #[test]
    fn test_default() {
        VolumeWeightedAveragePrice::default();
    }

    #[test]
    fn test_display() {
        let obv = VolumeWeightedAveragePrice::new();
        assert_eq!(format!("{}", obv), "OBV");
    }
}
