//! # scuffle-bootstrap
//!
//! A utility crate for creating binaries.
//!
//! Refer to [`Global`], [`Service`], and [`main`] for more information.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use std::sync::Arc;
//!
//! /// Our global state
//! struct Global;
//!
//! // Required by the signal service
//! impl scuffle_signal::SignalConfig for Global {}
//!
//! impl scuffle_bootstrap::global::GlobalWithoutConfig for Global {
//!     async fn init() -> anyhow::Result<Arc<Self>> {
//!         Ok(Arc::new(Self))
//!     }
//! }
//!
//! /// Our own custom service
//! struct MySvc;
//!
//! impl scuffle_bootstrap::service::Service<Global> for MySvc {
//!     async fn run(self, global: Arc<Global>, ctx: scuffle_context::Context) -> anyhow::Result<()> {
//!         # let _ = global;
//!         println!("running");
//!
//!         // Do some work here
//!
//!         // Wait for the context to be cacelled by the signal service
//!         ctx.done().await;
//!         Ok(())
//!     }
//! }
//!
//! // This generates the main function which runs all the services
//! scuffle_bootstrap::main! {
//!     Global {
//!         scuffle_signal::SignalSvc,
//!         MySvc,
//!     }
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the [MIT](./LICENSE.MIT) or
//! [Apache-2.0](./LICENSE.Apache-2.0) license. You can choose between one of
//! them if you use this work.
//!
//! `SPDX-License-Identifier: MIT OR Apache-2.0`
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

pub mod config;
pub mod global;
pub mod service;

pub use config::{ConfigParser, EmptyConfig};
pub use global::Global;
pub use scuffle_bootstrap_derive::main;
pub use service::Service;

#[doc(hidden)]
pub mod prelude {
    pub use {anyhow, futures, scuffle_context, tokio};
}

// Note: Tests are disabled due to a problem with cargo caching

// #[cfg(test)]
// #[cfg_attr(all(test, coverage_nightly), coverage(off))]
// mod tests {
//     #[test]
//     fn main_test() {
//         insta::assert_snapshot!(postcompile::compile! {
//             use std::sync::Arc;

//             use scuffle_bootstrap::main;

//             struct TestGlobal;

//             impl scuffle_signal::SignalConfig for TestGlobal {}

//             impl scuffle_bootstrap::global::GlobalWithoutConfig for
// TestGlobal {                 async fn init() -> anyhow::Result<Arc<Self>> {
//                     Ok(Arc::new(Self))
//                 }
//             }

//             main! {
//                 TestGlobal {
//                     scuffle_signal::SignalSvc,
//                 }
//             }
//         });
//     }

//     #[test]
//     fn main_test_custom_service() {
//         insta::assert_snapshot!(postcompile::compile! {
//             use std::sync::Arc;

//             use scuffle_bootstrap::main;

//             struct TestGlobal;

//             impl scuffle_signal::SignalConfig for TestGlobal {}

//             impl scuffle_bootstrap::global::GlobalWithoutConfig for
// TestGlobal {                 async fn init() -> anyhow::Result<Arc<Self>> {
//                     Ok(Arc::new(Self))
//                 }
//             }

//             struct MySvc;

//             impl scuffle_bootstrap::service::Service<TestGlobal> for MySvc {
//                 async fn run(self, _: Arc<TestGlobal>, _:
// scuffle_context::Context) -> anyhow::Result<()> {
// println!("running");                     Ok(())
//                 }
//             }

//             main! {
//                 TestGlobal {
//                     scuffle_signal::SignalSvc,
//                     MySvc,
//                 }
//             }
//         });
//     }
// }
