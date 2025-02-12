use nutype_enum::{bitwise_enum, nutype_enum};

use crate::ffi::*;

nutype_enum! {
    /// Packet flags used in FFmpeg's `AVPacket`.
    ///
    /// These flags describe metadata about a packet, such as whether it is a keyframe or corrupt.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/avcodec_8h.html>
    pub enum AVPktFlags(i32) {
        /// This packet contains a **keyframe**.
        /// - **Used for**: Identifying keyframes in video streams.
        /// - **Binary representation**: `0b00001`
        /// - **Equivalent to**: `AV_PKT_FLAG_KEY`
        Key = AV_PKT_FLAG_KEY as i32,

        /// This packet is **corrupt**.
        /// - **Used for**: Marking damaged or incomplete data.
        /// - **Binary representation**: `0b00010`
        /// - **Equivalent to**: `AV_PKT_FLAG_CORRUPT`
        Corrupt = AV_PKT_FLAG_CORRUPT as i32,

        /// This packet should be **discarded**.
        /// - **Used for**: Frames that should be ignored by decoders.
        /// - **Binary representation**: `0b00100`
        /// - **Equivalent to**: `AV_PKT_FLAG_DISCARD`
        Discard = AV_PKT_FLAG_DISCARD as i32,

        /// This packet comes from a **trusted source**.
        /// - **Used for**: Security and validation checks.
        /// - **Binary representation**: `0b01000`
        /// - **Equivalent to**: `AV_PKT_FLAG_TRUSTED`
        Trusted = AV_PKT_FLAG_TRUSTED as i32,

        /// This packet is **disposable** (e.g., non-reference frames).
        /// - **Used for**: Frames that can be dropped without affecting playback.
        /// - **Binary representation**: `0b10000`
        /// - **Equivalent to**: `AV_PKT_FLAG_DISPOSABLE`
        Disposable = AV_PKT_FLAG_DISPOSABLE as i32,
    }
}

bitwise_enum!(AVPktFlags);

impl PartialEq<i32> for AVPktFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
