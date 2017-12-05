//! ta is a Rust library for technical analysis. It provides number of technical indicators
//! that can be used to build trading strategies for stock markets, futures, forex, cryptocurrencies, etc.
//!
//! Every indicator is implemented as a data structure with fields, that define parameters and
//! state.
//!
//! Every indicator implements [Next<T>](trait.Next.html) and [Reset](trait.Reset.html) traits,
//! which are the core concept of the library.
//!
//! Since `Next<T>` is a generic trait, most of the indicators can work with both input types: `f64` and more complex
//! structures like [DataItem](struct.DataItem.html).
//!
//! # Example
//! ```
//! use ta::indicators::ExponentialMovingAverage;
//! use ta::Next;
//!
//! // it can return an error, when an invalid length is passed (e.g. 0)
//! let mut ema = ExponentialMovingAverage::new(3).unwrap();
//!
//! assert_eq!(ema.next(2.0), 2.0);
//! assert_eq!(ema.next(5.0), 3.5);
//! assert_eq!(ema.next(1.0), 2.25);
//! assert_eq!(ema.next(6.25), 4.25);
//! ```
#[macro_use]
extern crate error_chain;

#[cfg(test)]
#[macro_use]
mod test_helper;

mod helpers;

pub mod indicators;
pub mod errors;

mod traits;
pub use traits::*;

mod data_item;
pub use data_item::DataItem;
