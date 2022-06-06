use std::fmt;

use crate::errors::Result;
use crate::indicators::ExponentialMovingAverage;
use crate::{Close, Next, Period, Reset};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{LinearlyWeightedMovingAverage, RelativeMovingAverage, SimpleMovingAverage};

/// A moving average (MA).
///
/// This can be SMA, EMA, or RMA. Useful when you want to decide the type of moving average in runtime.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
#[doc(alias = "MA")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum MovingAverage {
    Simple(SimpleMovingAverage),
    Exponential(ExponentialMovingAverage),
    Relative(RelativeMovingAverage),
    Linear(LinearlyWeightedMovingAverage),
}

#[derive(Copy, Clone, Debug)]
pub enum MovingAverageType {
    Simple,
    Exponential,
    Relative,
    Linear,
}

impl MovingAverage {
    pub fn new(r#type: MovingAverageType, period: usize) -> Result<Self> {
        let ma = match r#type {
            MovingAverageType::Simple => Self::Simple(SimpleMovingAverage::new(period)?),
            MovingAverageType::Exponential => {
                Self::Exponential(ExponentialMovingAverage::new(period)?)
            }
            MovingAverageType::Relative => Self::Relative(RelativeMovingAverage::new(period)?),
            MovingAverageType::Linear => Self::Linear(LinearlyWeightedMovingAverage::new(period)?),
        };
        Ok(ma)
    }
}

impl Period for MovingAverage {
    fn period(&self) -> usize {
        match self {
            MovingAverage::Simple(x) => x.period(),
            MovingAverage::Exponential(x) => x.period(),
            MovingAverage::Relative(x) => x.period(),
            MovingAverage::Linear(x) => x.period(),
        }
    }
}

impl Next<f64> for MovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        match self {
            MovingAverage::Simple(x) => x.next(input),
            MovingAverage::Exponential(x) => x.next(input),
            MovingAverage::Relative(x) => x.next(input),
            MovingAverage::Linear(x) => x.next(input),
        }
    }
}

impl<T: Close> Next<&T> for MovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        match self {
            MovingAverage::Simple(x) => x.next(input),
            MovingAverage::Exponential(x) => x.next(input),
            MovingAverage::Relative(x) => x.next(input),
            MovingAverage::Linear(x) => x.next(input),
        }
    }
}

impl Reset for MovingAverage {
    fn reset(&mut self) {
        match self {
            MovingAverage::Simple(x) => x.reset(),
            MovingAverage::Exponential(x) => x.reset(),
            MovingAverage::Relative(x) => x.reset(),
            MovingAverage::Linear(x) => x.reset(),
        }
    }
}

impl Default for MovingAverage {
    fn default() -> Self {
        Self::new(MovingAverageType::Simple, 9).unwrap()
    }
}

impl fmt::Display for MovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MA({})", self.period())
    }
}
