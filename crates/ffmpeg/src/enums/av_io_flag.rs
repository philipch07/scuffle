use rusty_ffmpeg::ffi::*;
use nutype_enum::{nutype_enum, bitwise_enum};

nutype_enum! {
    /// I/O flags used in FFmpeg's `AVIOContext`.
    ///
    /// These flags define how a file or stream should be opened and accessed.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/avio_8h.html>
    pub enum AVIOFlag(i32) {
        /// Open the resource for reading.
        /// - **Used for**: Opening files or streams in read mode.
        /// - **Binary representation**: `0b0000000000000001`
        /// - **Equivalent to**: `AVIO_FLAG_READ`
        Read = AVIO_FLAG_READ as i32,

        /// Open the resource for writing.
        /// - **Used for**: Creating or overwriting files.
        /// - **Binary representation**: `0b0000000000000010`
        /// - **Equivalent to**: `AVIO_FLAG_WRITE`
        Write = AVIO_FLAG_WRITE as i32,

        /// Open the resource for both reading and writing.
        /// - **Used for**: Modifying an existing file or stream.
        /// - **Binary representation**: `0b0000000000000011`
        /// - **Equivalent to**: `AVIO_FLAG_READ_WRITE`
        ReadWrite = AVIO_FLAG_READ_WRITE as i32,

        /// Open the resource in non-blocking mode.
        /// - **Used for**: Asynchronous I/O operations.
        /// - **Binary representation**: `0b0000000000001000`
        /// - **Equivalent to**: `AVIO_FLAG_NONBLOCK`
        NonBlock = AVIO_FLAG_NONBLOCK as i32,

        /// Use direct I/O for lower-level access to storage.
        /// - **Used for**: Avoiding caching effects by the OS.
        /// - **Binary representation**: `0b1000000000000000`
        /// - **Equivalent to**: `AVIO_FLAG_DIRECT`
        Direct = AVIO_FLAG_DIRECT as i32,
    }
}

bitwise_enum!(AVIOFlag);

impl PartialEq<i32> for AVIOFlag {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
