//! # scuffle-metrics
//!
//! A wrapper around opentelemetry to provide a more ergonomic interface for
//! creating metrics.
//!
//! ## Status
//!
//! This crate is currently under development and is not yet stable.
//!
//! Unit tests are not yet fully implemented. Use at your own risk.
//!
//! ## License
//!
//! This project is licensed under the [MIT](./LICENSE.MIT) or
//! [Apache-2.0](./LICENSE.Apache-2.0) license. You can choose between one of
//! them if you use this work.
//!
//! `SPDX-License-Identifier: MIT OR Apache-2.0`

#[cfg(feature = "prometheus")]
/// A copy of the opentelemetry-prometheus crate, updated to work with the
/// latest version of opentelemetry.
pub mod prometheus;

#[doc(hidden)]
pub mod value;

pub mod collector;

pub use collector::{
    CounterF64, CounterU64, GaugeF64, GaugeI64, GaugeU64, HistogramF64, HistogramU64, UpDownCounterF64, UpDownCounterI64,
};
pub use opentelemetry;
pub use scuffle_metrics_derive::{metrics, MetricEnum};
