use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Linearly weighted moving average (LWMA).
///
/// LWMA is a moving average with the weights 1, 2, .. n.
/// The most recent item has the weight n and the oldest item has the weight 1.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Links
///
/// * [Linearly Weighted Moving Average, Investopedia](https://www.investopedia.com/terms/l/linearlyweightedmovingaverage.asp)
///
#[doc(alias = "LWMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct LinearlyWeightedMovingAverage {
    period: usize,
    index: usize,
    count: usize,
    weighted_sum: f64,
    sum: f64,
    deque: Box<[f64]>,
}

impl LinearlyWeightedMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                period,
                index: 0,
                count: 0,
                weighted_sum: 0.0,
                sum: 0.0,
                deque: vec![0.0; period].into_boxed_slice(),
            }),
        }
    }
}

impl Period for LinearlyWeightedMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for LinearlyWeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
            self.weighted_sum = self.weighted_sum + (self.count as f64) * input;
        } else {
            // 1*item[1] + 2*item[2] + ..     n*item[n]
            //             1*item[2] + .. (n-1)*item[n] + n*input
            self.weighted_sum = self.weighted_sum - self.sum + (self.period as f64) * input;
        }

        self.sum = self.sum - old_val + input;
        let weight = (self.count * (self.count + 1)) as f64 / 2.0;
        self.weighted_sum / weight
    }
}

impl<T: Close> Next<&T> for LinearlyWeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for LinearlyWeightedMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
        self.weighted_sum = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for LinearlyWeightedMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for LinearlyWeightedMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMA({})", self.period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(LinearlyWeightedMovingAverage);

    #[test]
    fn test_new() {
        assert!(LinearlyWeightedMovingAverage::new(0).is_err());
        assert!(LinearlyWeightedMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut sma = LinearlyWeightedMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 14.0 / 3.0);
        assert_eq!(sma.next(6.0), 32.0 / 6.0);
        assert_eq!(sma.next(6.0), 5.6);
        assert_eq!(sma.next(6.0), 5.9);
        assert_eq!(sma.next(6.0), 6.0);
        assert_eq!(sma.next(2.0), 4.4);
    }

    #[test]
    fn test_reset() {
        let mut sma = LinearlyWeightedMovingAverage::new(4).unwrap();
        assert_eq!(sma.next(4.0), 4.0);
        assert_eq!(sma.next(5.0), 14.0 / 3.0);
        assert_eq!(sma.next(6.0), 32.0 / 6.0);

        sma.reset();
        assert_eq!(sma.next(99.0), 99.0);
    }

    #[test]
    fn test_default() {
        LinearlyWeightedMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let sma = LinearlyWeightedMovingAverage::new(5).unwrap();
        assert_eq!(format!("{}", sma), "SMA(5)");
    }
}
