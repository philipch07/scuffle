use nutype_enum::nutype_enum;

use crate::ffi::*;

nutype_enum! {
    /// Represents the different media types supported by FFmpeg.
    ///
    /// See FFmpeg's `AVMediaType` in the official documentation:
    /// <https://ffmpeg.org/doxygen/trunk/group__lavu__misc.html#ga9a84bba4713dfced21a1a56163be1f48>
    pub enum AVMediaType(i32) {
        /// Unknown media type. Used when the type cannot be determined.
        /// Corresponds to `AVMEDIA_TYPE_UNKNOWN`.
        Unknown = AVMEDIA_TYPE_UNKNOWN,

        /// Video media type. Used for visual content such as movies or streams.
        /// Corresponds to `AVMEDIA_TYPE_VIDEO`.
        Video = AVMEDIA_TYPE_VIDEO,

        /// Audio media type. Represents sound or music data.
        /// Corresponds to `AVMEDIA_TYPE_AUDIO`.
        Audio = AVMEDIA_TYPE_AUDIO,

        /// Data media type. Typically used for supplementary or non-media data.
        /// Corresponds to `AVMEDIA_TYPE_DATA`.
        Data = AVMEDIA_TYPE_DATA,

        /// Subtitle media type. Represents textual or graphical subtitles.
        /// Corresponds to `AVMEDIA_TYPE_SUBTITLE`.
        Subtitle = AVMEDIA_TYPE_SUBTITLE,

        /// Attachment media type. Used for files attached to a media container (e.g., fonts for subtitles).
        /// Corresponds to `AVMEDIA_TYPE_ATTACHMENT`.
        Attachment = AVMEDIA_TYPE_ATTACHMENT,

        /// Special enumeration value representing the number of media types.
        /// Not an actual media type.
        /// Corresponds to `AVMEDIA_TYPE_NB`.
        Nb = AVMEDIA_TYPE_NB,
    }
}

impl PartialEq<i32> for AVMediaType {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
