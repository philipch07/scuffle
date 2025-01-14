//! # scuffle-metrics
//!
//! A wrapper around opentelemetry to provide a more ergonomic interface for
//! creating metrics.
//!
//! ## Example
//!
//! ```rust
//! #[scuffle_metrics::metrics]
//! mod example {
//!     use scuffle_metrics::{MetricEnum, collector::CounterU64};
//!
//!     #[derive(MetricEnum)]
//!     pub enum Kind {
//!         Http,
//!         Grpc,
//!     }
//!
//!     #[metrics(unit = "requests")]
//!     pub fn request(kind: Kind) -> CounterU64;
//! }
//!
//! // Increment the counter
//! example::request(example::Kind::Http).incr();
//! ```
//!
//! For details see [`metrics`].
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
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// A copy of the opentelemetry-prometheus crate, updated to work with the
/// latest version of opentelemetry.
#[cfg(feature = "prometheus")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus")))]
pub mod prometheus;

#[doc(hidden)]
pub mod value;

pub mod collector;

pub use collector::{
    CounterF64, CounterU64, GaugeF64, GaugeI64, GaugeU64, HistogramF64, HistogramU64, UpDownCounterF64, UpDownCounterI64,
};
pub use opentelemetry;
pub use scuffle_metrics_derive::{metrics, MetricEnum};

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    #[test]
    fn derive_enum() {
        insta::assert_snapshot!(postcompile::compile! {
            use scuffle_metrics::MetricEnum;

            #[derive(MetricEnum)]
            pub enum Kind {
                Http,
                Grpc,
            }
        });
    }

    // #[test]
    // fn derive_module() {
    //     insta::assert_snapshot!(postcompile::compile! {
    //         #[scuffle_metrics::metrics]
    //         mod example {
    //             use scuffle_metrics::{MetricEnum, collector::CounterU64};

    //             #[derive(MetricEnum)]
    //             pub enum Kind {
    //                 Http,
    //                 Grpc,
    //             }

    //             #[metrics(unit = "requests")]
    //             pub fn request(kind: Kind) -> CounterU64;
    //         }
    //     });
    // }
}
