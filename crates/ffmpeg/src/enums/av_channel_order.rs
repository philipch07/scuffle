use nutype_enum::nutype_enum;

use crate::ffi::*;

nutype_enum! {
    /// Audio channel ordering schemes used in FFmpeg's `AVChannelOrder`.
    ///
    /// This enum defines how channels are arranged in an audio stream, determining
    /// their order and mapping.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/channel__layout_8h.html>
    pub enum AVChannelOrder(u32) {
        /// Only the **channel count** is specified, without any further information
        /// about the **channel order**.
        /// - **Used for**: Unspecified channel layouts.
        /// - **Equivalent to**: `AV_CHANNEL_ORDER_UNSPEC`
        Unspecified = AV_CHANNEL_ORDER_UNSPEC,

        /// Channels are in the **native order** defined in `AVChannel` (up to 63 channels).
        /// - **Used for**: Standard layouts where channels are ordered as per the `AVChannel` enum.
        /// - **Equivalent to**: `AV_CHANNEL_ORDER_NATIVE`
        Native = AV_CHANNEL_ORDER_NATIVE,

        /// The channel order does not correspond to any predefined order and is stored
        /// as an **explicit map**.
        /// - **Used for**:
        ///   - Layouts with **64 or more channels**.
        ///   - Layouts with **empty/skipped** (`AV_CHAN_UNUSED`) channels at arbitrary positions.
        /// - **Example**: Custom surround sound layouts.
        /// - **Equivalent to**: `AV_CHANNEL_ORDER_CUSTOM`
        Custom = AV_CHANNEL_ORDER_CUSTOM,

        /// **Ambisonic channel order**, where each channel represents a **spherical harmonic**
        /// expansion component.
        ///
        /// **Channel arrangement (ACN - Ambisonic Channel Number)**:
        /// - Channel index **n** is mapped to spherical harmonic degree **l** and order **m**:
        ///   - `l = floor(sqrt(n))`
        ///   - `m = n - l * (l + 1)`
        /// - Conversely, given degree **l** and order **m**, the channel index is:
        ///   - `n = l * (l + 1) + m`
        ///
        /// **Normalization**: SN3D (Schmidt Semi-Normalization) as defined in **AmbiX format §2.1**.
        ///
        /// - **Used for**: **Ambisonic (3D spatial audio)** representations.
        /// - **Equivalent to**: `AV_CHANNEL_ORDER_AMBISONIC`
        Ambisonic = AV_CHANNEL_ORDER_AMBISONIC,

        /// **Number of channel orders** (internal use only).
        /// - **DO NOT USE** in applications.
        /// - **Equivalent to**: `FF_CHANNEL_ORDER_NB`
        Nb = FF_CHANNEL_ORDER_NB,
    }
}

impl PartialEq<u32> for AVChannelOrder {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl From<AVChannelOrder> for crate::ffi::AVChannelOrder {
    fn from(value: AVChannelOrder) -> Self {
        value.0
    }
}

impl From<crate::ffi::AVChannelOrder> for AVChannelOrder {
    fn from(value: crate::ffi::AVChannelOrder) -> Self {
        AVChannelOrder(value)
    }
}
