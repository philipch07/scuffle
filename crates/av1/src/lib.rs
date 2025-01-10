#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

mod config;
mod obu;

pub use config::AV1CodecConfigurationRecord;
pub use obu::{seq, ObuHeader, ObuType};
