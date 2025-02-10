mod input;
mod internal;
mod output;

/// A module that contains the channel implementation for io operations.
#[cfg(feature = "channel")]
#[cfg_attr(docsrs, doc(cfg(feature = "channel")))]
pub mod channel;

pub use input::*;
pub use output::*;
