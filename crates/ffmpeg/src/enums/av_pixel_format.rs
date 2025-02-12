use nutype_enum::nutype_enum;

use crate::ffi::*;

nutype_enum! {
    /// Pixel formats used in FFmpeg's `AVPixelFormat` enumeration.
    ///
    /// This enum represents different ways pixels can be stored in memory,
    /// including packed, planar, and hardware-accelerated formats.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/pixfmt_8h.html>
    pub enum AVPixelFormat(i32) {
        /// No pixel format specified or unknown format.
        /// Corresponds to `AV_PIX_FMT_NONE`.
        None = AV_PIX_FMT_NONE,

        /// Planar YUV 4:2:0 format, 12 bits per pixel.
        /// Each plane is stored separately, with 1 Cr & Cb sample per 2x2 Y samples.
        /// Corresponds to `AV_PIX_FMT_YUV420P`.
        Yuv420p = AV_PIX_FMT_YUV420P,

        /// Packed YUV 4:2:2 format, 16 bits per pixel.
        /// Stored as Y0 Cb Y1 Cr.
        /// Corresponds to `AV_PIX_FMT_Yuyv422`.
        Yuyv422 = AV_PIX_FMT_YUYV422,

        /// Packed RGB format, 8 bits per channel (24bpp).
        /// Stored as RGBRGB...
        /// Corresponds to `AV_PIX_FMT_RGB24`.
        Rgb24 = AV_PIX_FMT_RGB24,

        /// Packed BGR format, 8 bits per channel (24bpp).
        /// Stored as BGRBGR...
        /// Corresponds to `AV_PIX_FMT_BGR24`.
        Bgr24 = AV_PIX_FMT_BGR24,

        /// Planar YUV 4:2:2 format, 16 bits per pixel.
        /// Each plane is stored separately, with 1 Cr & Cb sample per 2x1 Y samples.
        /// Corresponds to `AV_PIX_FMT_YUV422P`.
        Yuv422p = AV_PIX_FMT_YUV422P,

        /// Planar YUV 4:4:4 format, 24 bits per pixel.
        /// Each plane is stored separately, with 1 Cr & Cb sample per 1x1 Y samples.
        /// Corresponds to `AV_PIX_FMT_YUV444P`.
        Yuv444p = AV_PIX_FMT_YUV444P,

        /// 8-bit grayscale format, 8 bits per pixel.
        /// Corresponds to `AV_PIX_FMT_GRAY8`.
        Gray8 = AV_PIX_FMT_GRAY8,

        /// 1-bit monochrome format, 0 is white, 1 is black.
        /// Pixels are stored in bytes, ordered from the most significant bit.
        /// Corresponds to `AV_PIX_FMT_MonoWhite`.
        MonoWhite = AV_PIX_FMT_MONOWHITE,

        /// 1-bit monochrome format, 0 is black, 1 is white.
        /// Pixels are stored in bytes, ordered from the most significant bit.
        /// Corresponds to `AV_PIX_FMT_MonoBlack`.
        MonoBlack = AV_PIX_FMT_MONOBLACK,

        /// Packed RGB 5:6:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB565BE`
        Rgb565Be = AV_PIX_FMT_RGB565BE,

        /// Packed RGB 5:6:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB565LE`
        Rgb565Le = AV_PIX_FMT_RGB565LE,

        /// Packed RGB 5:5:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB555BE`
        Rgb555Be = AV_PIX_FMT_RGB555BE,

        /// Packed RGB 5:5:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB555LE`
        Rgb555Le = AV_PIX_FMT_RGB555LE,

        /// Packed BGR 5:6:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGR565BE`
        Bgr565Be = AV_PIX_FMT_BGR565BE,

        /// Packed BGR 5:6:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGR565LE`
        Bgr565Le = AV_PIX_FMT_BGR565LE,

        /// Packed BGR 5:5:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGR555BE`
        Bgr555Be = AV_PIX_FMT_BGR555BE,

        /// Packed BGR 5:5:5 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGR555LE`
        Bgr555Le = AV_PIX_FMT_BGR555LE,

        /// Planar YUV 4:2:0 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV420P16BE`
        Yuv420p16Be = AV_PIX_FMT_YUV420P16BE,

        /// Planar YUV 4:2:0 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV420P16LE`
        Yuv420p16Le = AV_PIX_FMT_YUV420P16LE,

        /// Planar YUV 4:2:2 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV422P16BE`
        Yuv422p16Be = AV_PIX_FMT_YUV422P16BE,

        /// Planar YUV 4:2:2 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV422P16LE`
        Yuv422p16Le = AV_PIX_FMT_YUV422P16LE,

        /// Planar YUV 4:4:4 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV444P16BE`
        Yuv444p16Be = AV_PIX_FMT_YUV444P16BE,

        /// Planar YUV 4:4:4 format, 16 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_YUV444P16LE`
        Yuv444p16Le = AV_PIX_FMT_YUV444P16LE,

        /// Packed RGB 16:16:16 format, 48 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB48BE`
        Rgb48Be = AV_PIX_FMT_RGB48BE,

        /// Packed RGB 16:16:16 format, 48 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGB48LE`
        Rgb48Le = AV_PIX_FMT_RGB48LE,

        /// Packed RGBA 16:16:16:16 format, 64 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGBA64BE`
        Rgba64Be = AV_PIX_FMT_RGBA64BE,

        /// Packed RGBA 16:16:16:16 format, 64 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_RGBA64LE`
        Rgba64Le = AV_PIX_FMT_RGBA64LE,

        /// Packed BGRA 16:16:16:16 format, 64 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGRA64BE`
        Bgra64Be = AV_PIX_FMT_BGRA64BE,

        /// Packed BGRA 16:16:16:16 format, 64 bits per pixel.
        /// Corresponds to: `AV_PIX_FMT_BGRA64LE`
        Bgra64Le = AV_PIX_FMT_BGRA64LE,

        /// Hardware-accelerated format through VA-API.
        /// Corresponds to `AV_PIX_FMT_VAAPI`.
        Vaapi = AV_PIX_FMT_VAAPI,

        /// Planar GBR format, 4:4:4 subsampling.
        /// Corresponds to `AV_PIX_FMT_GBRP`.
        Gbrp = AV_PIX_FMT_GBRP,

        /// Format count, not an actual pixel format.
        /// Used internally by FFmpeg.
        /// Corresponds to `AV_PIX_FMT_NB`.
        Nb = AV_PIX_FMT_NB,
    }
}

impl PartialEq<i32> for AVPixelFormat {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<u32> for AVPixelFormat {
    fn from(value: u32) -> Self {
        AVPixelFormat(value as i32)
    }
}

impl From<AVPixelFormat> for u32 {
    fn from(value: AVPixelFormat) -> Self {
        value.0 as u32
    }
}
