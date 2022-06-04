use std::fmt;

use crate::errors::{Result, TaError};
use crate::indicators::ExponentialMovingAverage;
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A relative moving average (RMA).
///
/// This is an exponential moving average (EMA) with a different alpha value.
/// RSI and ATR use RMA by default.
///
///
/// # Formula
///
/// α = 1 / period
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use ta::indicators::RelativeMovingAverage;
/// use ta::Next;
///
/// let mut ema = RelativeMovingAverage::new(3).unwrap();
/// assert_eq!(ema.next(2.0), 2.0);
/// assert_eq!(ema.next(5.0), 3.0);
/// assert_eq!(ema.next(1.0), 7.0/3.0);
/// assert_approx_eq::assert_approx_eq!(ema.next(6.25), 3.6);
/// ```
///

#[doc(alias = "RMA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RelativeMovingAverage {
    ema: ExponentialMovingAverage,
}

impl RelativeMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self {
                ema: ExponentialMovingAverage::with_k(period, 1.0 / period as f64).unwrap(),
            }),
        }
    }
}

impl Period for RelativeMovingAverage {
    fn period(&self) -> usize {
        self.ema.period()
    }
}

impl Next<f64> for RelativeMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(input)
    }
}

impl<T: Close> Next<&T> for RelativeMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(input)
    }
}

impl Reset for RelativeMovingAverage {
    fn reset(&mut self) {
        self.ema.reset()
    }
}

impl Default for RelativeMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for RelativeMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RMA({})", self.ema.period())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::*;

    test_indicator!(RelativeMovingAverage);

    #[test]
    fn test_new() {
        assert!(RelativeMovingAverage::new(0).is_err());
        assert!(RelativeMovingAverage::new(1).is_ok());
    }

    #[test]
    fn test_next() {
        let mut rma = RelativeMovingAverage::new(3).unwrap();

        assert_eq!(rma.next(2.0), 2.0);
        assert_eq!(rma.next(5.0), 3.0);

        assert_eq!(rma.next(1.0), 7.0 / 3.0);
        assert_approx_eq::assert_approx_eq!(rma.next(6.25), 14.0 / 9.0 + 25.0 / 12.0);

        let mut rma = RelativeMovingAverage::new(3).unwrap();
        let bar1 = Bar::new().close(2);
        let bar2 = Bar::new().close(5);
        assert_eq!(rma.next(&bar1), 2.0);
        assert_eq!(rma.next(&bar2), 3.0);
    }

    #[test]
    fn test_reset() {
        let mut rma = RelativeMovingAverage::new(5).unwrap();

        assert_eq!(rma.next(4.0), 4.0);
        rma.next(10.0);
        rma.next(15.0);
        rma.next(20.0);
        assert_ne!(rma.next(4.0), 4.0);

        rma.reset();
        assert_eq!(rma.next(4.0), 4.0);
    }

    #[test]
    fn test_default() {
        RelativeMovingAverage::default();
    }

    #[test]
    fn test_display() {
        let rma = RelativeMovingAverage::new(7).unwrap();
        assert_eq!(format!("{}", rma), "RMA(7)");
    }
}
