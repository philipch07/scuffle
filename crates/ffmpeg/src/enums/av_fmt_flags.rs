use nutype_enum::{bitwise_enum, nutype_enum};

use crate::ffi::*;

nutype_enum! {
    /// Format flags used in FFmpeg's `AVFormatContext` configuration.
    ///
    /// These flags are **format-specific capabilities** that describe the inherent
    /// characteristics and limitations of a format (container). They are read-only
    /// properties that indicate what features a format supports or doesn't support.
    ///
    /// For example, `NoFile` indicates the format doesn't need a regular file (like
    /// network protocols), while `GlobalHeader` indicates the format uses global codec
    /// headers.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/avformat_8h.html>
    pub enum AVFormatFlags(i32) {
        /// The format does not require a file to be opened explicitly.
        /// - **Used for**: Protocol-based formats like `rtmp://`, `http://`
        /// - **Equivalent to**: `AVFMT_NOFILE`
        NoFile = AVFMT_NOFILE as i32,

        /// Requires a numbered sequence of files (e.g., `%03d` in filenames).
        /// - **Used for**: Image sequences, segment-based formats.
        /// - **Equivalent to**: `AVFMT_NEEDNUMBER`
        NeedNumber = AVFMT_NEEDNUMBER as i32,

        /// The format is experimental and may be subject to changes.
        /// - **Used for**: Newer formats that are not yet stable.
        /// - **Equivalent to**: `AVFMT_EXPERIMENTAL`
        Experimental = AVFMT_EXPERIMENTAL as i32,

        /// Displays stream identifiers when logging or printing metadata.
        /// - **Equivalent to**: `AVFMT_SHOW_IDS`
        ShowIds = AVFMT_SHOW_IDS as i32,

        /// Uses a global header instead of individual packet headers.
        /// - **Used for**: Codecs that require an extradata header (e.g., H.264, AAC in MP4).
        /// - **Equivalent to**: `AVFMT_GLOBALHEADER`
        GlobalHeader = AVFMT_GLOBALHEADER as i32,

        /// The format does not store timestamps.
        /// - **Used for**: Raw formats (e.g., raw audio, raw video).
        /// - **Equivalent to**: `AVFMT_NOTIMESTAMPS`
        NoTimestamps = AVFMT_NOTIMESTAMPS as i32,

        /// The format has a generic index.
        /// - **Used for**: Formats that require seeking but don't use timestamp-based indexing.
        /// - **Equivalent to**: `AVFMT_GENERIC_INDEX`
        GenericIndex = AVFMT_GENERIC_INDEX as i32,

        /// The format supports discontinuous timestamps.
        /// - **Used for**: Live streams where timestamps may reset (e.g., HLS, RTSP).
        /// - **Equivalent to**: `AVFMT_TS_DISCONT`
        TsDiscontinuous = AVFMT_TS_DISCONT as i32,

        /// The format supports variable frame rates.
        /// - **Used for**: Video formats where frame duration varies (e.g., MKV, MP4).
        /// - **Equivalent to**: `AVFMT_VARIABLE_FPS`
        VariableFps = AVFMT_VARIABLE_FPS as i32,

        /// The format does not store dimensions (width & height).
        /// - **Used for**: Audio-only formats, raw formats.
        /// - **Equivalent to**: `AVFMT_NODIMENSIONS`
        NoDimensions = AVFMT_NODIMENSIONS as i32,

        /// The format does not contain any stream information.
        /// - **Used for**: Metadata-only containers.
        /// - **Equivalent to**: `AVFMT_NOSTREAMS`
        NoStreams = AVFMT_NOSTREAMS as i32,

        /// The format does not support binary search for seeking.
        /// - **Used for**: Formats where linear scanning is required (e.g., live streams).
        /// - **Equivalent to**: `AVFMT_NOBINSEARCH`
        NoBinarySearch = AVFMT_NOBINSEARCH as i32,

        /// The format does not support generic stream search.
        /// - **Used for**: Specialized formats that require specific handling.
        /// - **Equivalent to**: `AVFMT_NOGENSEARCH`
        NoGenericSearch = AVFMT_NOGENSEARCH as i32,

        /// The format does not support byte-based seeking.
        /// - **Used for**: Formats that only support timestamp-based seeking.
        /// - **Equivalent to**: `AVFMT_NO_BYTE_SEEK`
        NoByteSeek = AVFMT_NO_BYTE_SEEK as i32,

        /// Allows flushing of buffered data.
        /// - **Used for**: Streaming formats that support mid-stream flushing.
        /// - **Equivalent to**: `AVFMT_ALLOW_FLUSH`
        AllowFlush = AVFMT_ALLOW_FLUSH as i32,

        /// The format does not require strict timestamp ordering.
        /// - **Used for**: Formats where out-of-order timestamps are common.
        /// - **Equivalent to**: `AVFMT_TS_NONSTRICT`
        TsNonStrict = AVFMT_TS_NONSTRICT as i32,

        /// The format allows negative timestamps.
        /// - **Used for**: Certain formats that support negative PTS/DTS.
        /// - **Equivalent to**: `AVFMT_TS_NEGATIVE`
        TsNegative = AVFMT_TS_NEGATIVE as i32,

        /// Seeks are performed relative to presentation timestamps (PTS).
        /// - **Used for**: Formats that use PTS instead of DTS for seeking.
        /// - **Equivalent to**: `AVFMT_SEEK_TO_PTS`
        SeekToPts = AVFMT_SEEK_TO_PTS as i32,
    }
}

bitwise_enum!(AVFormatFlags);

impl PartialEq<i32> for AVFormatFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVFormatFlags {
    fn from(value: u32) -> Self {
        AVFormatFlags(value as i32)
    }
}

impl From<AVFormatFlags> for u32 {
    fn from(value: AVFormatFlags) -> Self {
        value.0 as u32
    }
}

nutype_enum! {
    /// Format flags used in FFmpeg's `AVFormatContext`.
    ///
    /// These flags are **user-configurable options** that control how FFmpeg should
    /// behave when reading or writing media. Unlike `AVFormatFlags` which describe
    /// format capabilities, these flags modify the runtime behavior of demuxers and
    /// muxers.
    ///
    /// For example, `GenPts` tells FFmpeg to generate missing timestamps, while
    /// `FastSeek` enables optimized seeking behavior.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/avformat_8h.html>
    pub enum AVFmtFlags(i32) {
        /// Generate **Presentation Timestamps (PTS)** if they are missing.
        /// - **Used for**: Formats that may not provide timestamps.
        /// - **Binary representation**: `0b0000000000000001`
        /// - **Equivalent to**: `AVFMT_FLAG_GENPTS`
        GenPts = AVFMT_FLAG_GENPTS as i32,

        /// Ignore the index when seeking.
        /// - **Used for**: Faster seeking in formats that rely on indexes.
        /// - **Binary representation**: `0b0000000000000010`
        /// - **Equivalent to**: `AVFMT_FLAG_IGNIDX`
        IgnoreIndex = AVFMT_FLAG_IGNIDX as i32,

        /// Open input in **non-blocking mode**.
        /// - **Used for**: Asynchronous reading.
        /// - **Binary representation**: `0b0000000000000100`
        /// - **Equivalent to**: `AVFMT_FLAG_NONBLOCK`
        NonBlock = AVFMT_FLAG_NONBLOCK as i32,

        /// Ignore **Decoding Timestamps (DTS)**.
        /// - **Used for**: Cases where only PTS is needed.
        /// - **Binary representation**: `0b0000000000001000`
        /// - **Equivalent to**: `AVFMT_FLAG_IGNDTS`
        IgnoreDts = AVFMT_FLAG_IGNDTS as i32,

        /// Do not fill in missing information in streams.
        /// - **Used for**: Avoiding unwanted automatic corrections.
        /// - **Binary representation**: `0b0000000000010000`
        /// - **Equivalent to**: `AVFMT_FLAG_NOFILLIN`
        NoFillIn = AVFMT_FLAG_NOFILLIN as i32,

        /// Do not parse frames.
        /// - **Used for**: Formats where parsing is unnecessary.
        /// - **Binary representation**: `0b0000000000100000`
        /// - **Equivalent to**: `AVFMT_FLAG_NOPARSE`
        NoParse = AVFMT_FLAG_NOPARSE as i32,

        /// Disable internal buffering.
        /// - **Used for**: Real-time applications requiring low latency.
        /// - **Binary representation**: `0b0000000001000000`
        /// - **Equivalent to**: `AVFMT_FLAG_NOBUFFER`
        NoBuffer = AVFMT_FLAG_NOBUFFER as i32,

        /// Use **custom I/O** instead of standard file I/O.
        /// - **Used for**: Implementing custom read/write operations.
        /// - **Binary representation**: `0b0000000010000000`
        /// - **Equivalent to**: `AVFMT_FLAG_CUSTOM_IO`
        CustomIO = AVFMT_FLAG_CUSTOM_IO as i32,

        /// Discard **corrupt** frames.
        /// - **Used for**: Ensuring only valid frames are processed.
        /// - **Binary representation**: `0b0000000100000000`
        /// - **Equivalent to**: `AVFMT_FLAG_DISCARD_CORRUPT`
        DiscardCorrupt = AVFMT_FLAG_DISCARD_CORRUPT as i32,

        /// **Flush packets** after writing.
        /// - **Used for**: Streaming to avoid buffering delays.
        /// - **Binary representation**: `0b0000001000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_FLUSH_PACKETS`
        FlushPackets = AVFMT_FLAG_FLUSH_PACKETS as i32,

        /// Ensure **bit-exact** output.
        /// - **Used for**: Regression testing, avoiding encoding variations.
        /// - **Binary representation**: `0b0000010000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_BITEXACT`
        BitExact = AVFMT_FLAG_BITEXACT as i32,

        /// Sort packets by **Decoding Timestamp (DTS)**.
        /// - **Used for**: Ensuring ordered input.
        /// - **Binary representation**: `0b0001000000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_SORT_DTS`
        SortDts = AVFMT_FLAG_SORT_DTS as i32,

        /// Enable **fast seeking**.
        /// - **Used for**: Improving seek performance in large files.
        /// - **Binary representation**: `0b0010000000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_FAST_SEEK`
        FastSeek = AVFMT_FLAG_FAST_SEEK as i32,

        /// Stop **decoding at the shortest stream**.
        /// - **Used for**: Ensuring synchronization in multi-stream files.
        /// - **Binary representation**: `0b0100000000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_SHORTEST`
        Shortest = AVFMT_FLAG_SHORTEST as i32,

        /// **Automatically apply bitstream filters**.
        /// - **Used for**: Simplifying format conversions.
        /// - **Binary representation**: `0b1000000000000000`
        /// - **Equivalent to**: `AVFMT_FLAG_AUTO_BSF`
        AutoBsf = AVFMT_FLAG_AUTO_BSF as i32,
    }
}

bitwise_enum!(AVFmtFlags);

impl PartialEq<i32> for AVFmtFlags {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVFmtFlags {
    fn from(value: u32) -> Self {
        AVFmtFlags(value as i32)
    }
}

impl From<AVFmtFlags> for u32 {
    fn from(value: AVFmtFlags) -> Self {
        value.0 as u32
    }
}
