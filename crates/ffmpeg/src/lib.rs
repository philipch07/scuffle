//! A crate designed to provide a simple interface to the native ffmpeg c-bindings.
//!
//! ## Why do we need this?
//!
//! This crate aims to provide a simple-safe interface to the native ffmpeg c-bindings.
//!
//! ## How is this different from other ffmpeg crates?
//!
//! The other main ffmpeg crate is [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg).
//!
//! This crate adds a few features and has a safer API. Notably it adds the ability to provide a an in-memory decode / encode buffer.
//!
//! ## Status
//!
//! This crate is currently under development and is not yet stable.
//!
//! Unit tests are not yet fully implemented. Use at your own risk.
//!
//! ## License
//!
//! This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
//! You can choose between one of them if you use this work.
//!
//! `SPDX-License-Identifier: MIT OR Apache-2.0`
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

pub mod codec;
pub mod consts;
pub mod decoder;
pub mod dict;
pub mod encoder;
pub mod error;
pub mod filter_graph;
pub mod frame;
pub mod io;
pub mod limiter;
pub mod log;
pub mod packet;
pub mod scalar;
pub mod stream;
pub mod utils;

pub use ffmpeg_sys_next as ffi;

mod smart_object;
