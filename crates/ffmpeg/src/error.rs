use ffmpeg_sys_next::*;
use nutype_enum::nutype_enum;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FfmpegError {
    #[error("failed to allocate memory")]
    Alloc,
    #[error("ffmpeg error: {0}")]
    Code(#[from] FfmpegErrorCode),
    #[error("no decoder found")]
    NoDecoder,
    #[error("no encoder found")]
    NoEncoder,
    #[error("no stream found")]
    NoStream,
    #[error("no filter found")]
    NoFilter,
    #[error("no frame found")]
    NoFrame,
    #[error("invalid arguments: {0}")]
    Arguments(&'static str),
}

pub(crate) const AVERROR_EAGAIN: i32 = AVERROR(EAGAIN);

nutype_enum! {
    pub enum FfmpegErrorCode(i32) {
        EndOfFile = AVERROR_EOF,
        InvalidData = AVERROR_INVALIDDATA,
        MuxerNotFound = AVERROR_MUXER_NOT_FOUND,
        OptionNotFound = AVERROR_OPTION_NOT_FOUND,
        PatchWelcome = AVERROR_PATCHWELCOME,
        ProtocolNotFound = AVERROR_PROTOCOL_NOT_FOUND,
        StreamNotFound = AVERROR_STREAM_NOT_FOUND,
        BitstreamFilterNotFound = AVERROR_BSF_NOT_FOUND,
        Bug = AVERROR_BUG,
        Eof = AVERROR_EOF,
        Eagain = AVERROR_EAGAIN,
        BufferTooSmall = AVERROR_BUFFER_TOO_SMALL,
        DecoderNotFound = AVERROR_DECODER_NOT_FOUND,
        DemuxerNotFound = AVERROR_DEMUXER_NOT_FOUND,
        EncoderNotFound = AVERROR_ENCODER_NOT_FOUND,
        Exit = AVERROR_EXIT,
        External = AVERROR_EXTERNAL,
        FilterNotFound = AVERROR_FILTER_NOT_FOUND,
        HttpBadRequest = AVERROR_HTTP_BAD_REQUEST,
        HttpForbidden = AVERROR_HTTP_FORBIDDEN,
        HttpNotFound = AVERROR_HTTP_NOT_FOUND,
        HttpOther4xx = AVERROR_HTTP_OTHER_4XX,
        HttpServerError = AVERROR_HTTP_SERVER_ERROR,
        HttpUnauthorized = AVERROR_HTTP_UNAUTHORIZED,
        Bug2 = AVERROR_BUG2,
        Unknown = AVERROR_UNKNOWN,
    }
}

impl FfmpegErrorCode {
    /// Returns the result of the error code.
    pub const fn result(self) -> Result<i32, FfmpegError> {
        match self {
            code if code.is_success() => Ok(code.0),
            _ => Err(FfmpegError::Code(self)),
        }
    }

    pub const fn is_success(self) -> bool {
        self.0 >= 0
    }
}

impl std::fmt::Display for FfmpegErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::EndOfFile => write!(f, "end of file"),
            Self::InvalidData => write!(f, "invalid data"),
            Self::MuxerNotFound => write!(f, "muxer not found"),
            Self::OptionNotFound => write!(f, "option not found"),
            Self::PatchWelcome => write!(f, "patch welcome"),
            Self::ProtocolNotFound => write!(f, "protocol not found"),
            Self::StreamNotFound => write!(f, "stream not found"),
            Self::BitstreamFilterNotFound => write!(f, "bitstream filter not found"),
            Self::Bug => write!(f, "bug"),
            Self::BufferTooSmall => write!(f, "buffer too small"),
            Self::DecoderNotFound => write!(f, "decoder not found"),
            Self::DemuxerNotFound => write!(f, "demuxer not found"),
            Self::EncoderNotFound => write!(f, "encoder not found"),
            Self::Exit => write!(f, "exit"),
            Self::External => write!(f, "external"),
            Self::FilterNotFound => write!(f, "filter not found"),
            Self::HttpBadRequest => write!(f, "http bad request"),
            Self::HttpForbidden => write!(f, "http forbidden"),
            Self::HttpNotFound => write!(f, "http not found"),
            Self::HttpOther4xx => write!(f, "http other 4xx"),
            Self::HttpServerError => write!(f, "http server error"),
            Self::HttpUnauthorized => write!(f, "http unauthorized"),
            Self::Bug2 => write!(f, "bug2"),
            Self::Unknown => write!(f, "unknown"),
            Self(ec) => write!(f, "unknown error code: {ec}"),
        }
    }
}

impl std::error::Error for FfmpegErrorCode {}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use super::{FfmpegError, FfmpegErrorCode};
    use crate::error::*;

    #[test]
    fn test_ffmpeg_error_code_display() {
        let cases = [
            (FfmpegErrorCode::EndOfFile, "end of file"),
            (FfmpegErrorCode::InvalidData, "invalid data"),
            (FfmpegErrorCode::MuxerNotFound, "muxer not found"),
            (FfmpegErrorCode::OptionNotFound, "option not found"),
            (FfmpegErrorCode::PatchWelcome, "patch welcome"),
            (FfmpegErrorCode::ProtocolNotFound, "protocol not found"),
            (FfmpegErrorCode::StreamNotFound, "stream not found"),
            (FfmpegErrorCode::BitstreamFilterNotFound, "bitstream filter not found"),
            (FfmpegErrorCode::Bug, "bug"),
            (FfmpegErrorCode::BufferTooSmall, "buffer too small"),
            (FfmpegErrorCode::DecoderNotFound, "decoder not found"),
            (FfmpegErrorCode::DemuxerNotFound, "demuxer not found"),
            (FfmpegErrorCode::EncoderNotFound, "encoder not found"),
            (FfmpegErrorCode::Exit, "exit"),
            (FfmpegErrorCode::External, "external"),
            (FfmpegErrorCode::FilterNotFound, "filter not found"),
            (FfmpegErrorCode::HttpBadRequest, "http bad request"),
            (FfmpegErrorCode::HttpForbidden, "http forbidden"),
            (FfmpegErrorCode::HttpNotFound, "http not found"),
            (FfmpegErrorCode::HttpOther4xx, "http other 4xx"),
            (FfmpegErrorCode::HttpServerError, "http server error"),
            (FfmpegErrorCode::HttpUnauthorized, "http unauthorized"),
            (FfmpegErrorCode::Bug2, "bug2"),
            (FfmpegErrorCode::Unknown, "unknown"),
            (FfmpegErrorCode(123), "unknown error code: 123"),
        ];

        for (code, expected) in cases {
            assert_eq!(code.to_string(), expected);
        }
    }

    #[test]
    fn test_ffmpeg_error_code_from_i32() {
        // Define constants that map to the FfmpegErrorCode variants
        const TEST_CASES: &[(i32, FfmpegErrorCode)] = &[
            (AVERROR_EOF, FfmpegErrorCode::EndOfFile),
            (AVERROR_INVALIDDATA, FfmpegErrorCode::InvalidData),
            (AVERROR_MUXER_NOT_FOUND, FfmpegErrorCode::MuxerNotFound),
            (AVERROR_OPTION_NOT_FOUND, FfmpegErrorCode::OptionNotFound),
            (AVERROR_PATCHWELCOME, FfmpegErrorCode::PatchWelcome),
            (AVERROR_PROTOCOL_NOT_FOUND, FfmpegErrorCode::ProtocolNotFound),
            (AVERROR_STREAM_NOT_FOUND, FfmpegErrorCode::StreamNotFound),
            (AVERROR_BSF_NOT_FOUND, FfmpegErrorCode::BitstreamFilterNotFound),
            (AVERROR_BUG, FfmpegErrorCode::Bug),
            (AVERROR_BUFFER_TOO_SMALL, FfmpegErrorCode::BufferTooSmall),
            (AVERROR_DECODER_NOT_FOUND, FfmpegErrorCode::DecoderNotFound),
            (AVERROR_DEMUXER_NOT_FOUND, FfmpegErrorCode::DemuxerNotFound),
            (AVERROR_ENCODER_NOT_FOUND, FfmpegErrorCode::EncoderNotFound),
            (AVERROR_EXIT, FfmpegErrorCode::Exit),
            (AVERROR_EXTERNAL, FfmpegErrorCode::External),
            (AVERROR_FILTER_NOT_FOUND, FfmpegErrorCode::FilterNotFound),
            (AVERROR_HTTP_BAD_REQUEST, FfmpegErrorCode::HttpBadRequest),
            (AVERROR_HTTP_FORBIDDEN, FfmpegErrorCode::HttpForbidden),
            (AVERROR_HTTP_NOT_FOUND, FfmpegErrorCode::HttpNotFound),
            (AVERROR_HTTP_OTHER_4XX, FfmpegErrorCode::HttpOther4xx),
            (AVERROR_HTTP_SERVER_ERROR, FfmpegErrorCode::HttpServerError),
            (AVERROR_HTTP_UNAUTHORIZED, FfmpegErrorCode::HttpUnauthorized),
            (AVERROR_BUG2, FfmpegErrorCode::Bug2),
            (AVERROR_UNKNOWN, FfmpegErrorCode::Unknown),
        ];

        // Test each case
        for &(value, expected) in TEST_CASES {
            let result: FfmpegErrorCode = value.into();
            assert_eq!(result, expected, "Failed for value: {value}");
        }

        // Test an unknown error case
        let unknown_value = 9999;
        let result: FfmpegErrorCode = unknown_value.into();
        assert_eq!(
            result,
            FfmpegErrorCode(unknown_value),
            "Failed for unknown value: {unknown_value}"
        );
    }

    #[test]
    fn test_ffmpeg_error_display() {
        let cases = [
            (FfmpegError::Alloc, "failed to allocate memory"),
            (FfmpegError::Code(FfmpegErrorCode::EndOfFile), "ffmpeg error: end of file"),
            (FfmpegError::NoDecoder, "no decoder found"),
            (FfmpegError::NoEncoder, "no encoder found"),
            (FfmpegError::NoStream, "no stream found"),
            (FfmpegError::NoFilter, "no filter found"),
            (FfmpegError::NoFrame, "no frame found"),
            (
                FfmpegError::Arguments("invalid argument example"),
                "invalid arguments: invalid argument example",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }
}
