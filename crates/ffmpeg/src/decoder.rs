use ffmpeg_sys_next::*;

use crate::codec::DecoderCodec;
use crate::error::{FfmpegError, AVERROR_EAGAIN};
use crate::frame::{Frame, VideoFrame};
use crate::packet::Packet;
use crate::smart_object::SmartPtr;
use crate::stream::Stream;

#[derive(Debug)]
pub enum Decoder {
    Video(VideoDecoder),
    Audio(AudioDecoder),
}

pub struct GenericDecoder {
    decoder: SmartPtr<AVCodecContext>,
}

/// Safety: `GenericDecoder` can be sent between threads.
unsafe impl Send for GenericDecoder {}

impl std::fmt::Debug for GenericDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("time_base", &self.time_base())
            .field("codec_type", &self.codec_type())
            .finish()
    }
}

pub struct VideoDecoder(GenericDecoder);

impl std::fmt::Debug for VideoDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoDecoder")
            .field("time_base", &self.time_base())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("pixel_format", &self.pixel_format())
            .field("frame_rate", &self.frame_rate())
            .field("sample_aspect_ratio", &self.sample_aspect_ratio())
            .finish()
    }
}

pub struct AudioDecoder(GenericDecoder);

impl std::fmt::Debug for AudioDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioDecoder")
            .field("time_base", &self.time_base())
            .field("sample_rate", &self.sample_rate())
            .field("channels", &self.channels())
            .field("sample_fmt", &self.sample_format())
            .finish()
    }
}

pub struct DecoderOptions {
    pub codec: Option<DecoderCodec>,
    pub thread_count: i32,
}

impl Default for DecoderOptions {
    fn default() -> Self {
        Self {
            codec: None,
            thread_count: 1,
        }
    }
}

impl Decoder {
    pub fn new(ist: &Stream) -> Result<Self, FfmpegError> {
        Self::with_options(ist, Default::default())
    }

    pub fn with_options(ist: &Stream, options: DecoderOptions) -> Result<Self, FfmpegError> {
        let Some(codec_params) = ist.codec_parameters() else {
            return Err(FfmpegError::NoDecoder);
        };

        let codec = options
            .codec
            .or_else(|| DecoderCodec::new(codec_params.codec_id))
            .ok_or(FfmpegError::NoDecoder)?;
        if codec.as_ptr().is_null() {
            return Err(FfmpegError::NoDecoder);
        }

        // Safety: `codec` is a valid pointer, also the pointer returned from
        // `avcodec_alloc_context3` is valid.
        let mut decoder =
            unsafe { SmartPtr::wrap_non_null(avcodec_alloc_context3(codec.as_ptr()), |ptr| avcodec_free_context(ptr)) }
                .ok_or(FfmpegError::Alloc)?;

        // Safety: `codec_params` is a valid pointer, and `decoder` is a valid pointer.
        let ret = unsafe { avcodec_parameters_to_context(decoder.as_mut_ptr(), codec_params) };
        if ret < 0 {
            return Err(FfmpegError::Code(ret.into()));
        }

        let decoder_mut = decoder.as_deref_mut_except();

        decoder_mut.pkt_timebase = ist.time_base();
        decoder_mut.time_base = ist.time_base();
        decoder_mut.thread_count = options.thread_count;

        if decoder_mut.codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO {
            // Even though we are upcasting `AVFormatContext` from a const pointer to a
            // mutable pointer, it is still safe becasuse av_guess_frame_rate does not use
            // the pointer to modify the `AVFormatContext`. https://github.com/FFmpeg/FFmpeg/blame/90bef6390fba02472141f299264331f68018a992/libavformat/avformat.c#L728
            // The function does not use the pointer at all, it only uses the `AVStream`
            // pointer to get the `AVRational`
            decoder_mut.framerate = unsafe {
                av_guess_frame_rate(
                    ist.format_context() as *const AVFormatContext as *mut AVFormatContext,
                    ist.as_ptr() as *mut AVStream,
                    std::ptr::null_mut(),
                )
            };
        }

        if matches!(
            decoder_mut.codec_type,
            AVMediaType::AVMEDIA_TYPE_VIDEO | AVMediaType::AVMEDIA_TYPE_AUDIO
        ) {
            // Safety: `codec` is a valid pointer, and `decoder` is a valid pointer.
            let ret = unsafe { avcodec_open2(decoder_mut, codec.as_ptr(), std::ptr::null_mut()) };
            if ret < 0 {
                return Err(FfmpegError::Code(ret.into()));
            }
        }

        Ok(match decoder_mut.codec_type {
            AVMediaType::AVMEDIA_TYPE_VIDEO => Self::Video(VideoDecoder(GenericDecoder { decoder })),
            AVMediaType::AVMEDIA_TYPE_AUDIO => Self::Audio(AudioDecoder(GenericDecoder { decoder })),
            _ => Err(FfmpegError::NoDecoder)?,
        })
    }
}

impl GenericDecoder {
    pub fn codec_type(&self) -> AVMediaType {
        self.decoder.as_deref_except().codec_type
    }

    pub fn time_base(&self) -> AVRational {
        self.decoder.as_deref_except().time_base
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<(), FfmpegError> {
        // Safety: `packet` is a valid pointer, and `self.decoder` is a valid pointer.
        let ret = unsafe { avcodec_send_packet(self.decoder.as_mut_ptr(), packet.as_ptr()) };

        match ret {
            0 => Ok(()),
            _ => Err(FfmpegError::Code(ret.into())),
        }
    }

    pub fn send_eof(&mut self) -> Result<(), FfmpegError> {
        // Safety: `self.decoder` is a valid pointer.
        let ret = unsafe { avcodec_send_packet(self.decoder.as_mut_ptr(), std::ptr::null()) };

        match ret {
            0 => Ok(()),
            _ => Err(FfmpegError::Code(ret.into())),
        }
    }

    pub fn receive_frame(&mut self) -> Result<Option<VideoFrame>, FfmpegError> {
        let mut frame = Frame::new()?;

        // Safety: `frame` is a valid pointer, and `self.decoder` is a valid pointer.
        let ret = unsafe { avcodec_receive_frame(self.decoder.as_mut_ptr(), frame.as_mut_ptr()) };

        match ret {
            AVERROR_EAGAIN | AVERROR_EOF => Ok(None),
            0 => {
                frame.set_time_base(self.decoder.as_deref_except().time_base);
                Ok(Some(frame.video()))
            }
            _ => Err(FfmpegError::Code(ret.into())),
        }
    }
}

impl VideoDecoder {
    pub fn width(&self) -> i32 {
        self.0.decoder.as_deref_except().width
    }

    pub fn height(&self) -> i32 {
        self.0.decoder.as_deref_except().height
    }

    pub fn pixel_format(&self) -> AVPixelFormat {
        self.0.decoder.as_deref_except().pix_fmt
    }

    pub fn frame_rate(&self) -> AVRational {
        self.0.decoder.as_deref_except().framerate
    }

    pub fn sample_aspect_ratio(&self) -> AVRational {
        self.0.decoder.as_deref_except().sample_aspect_ratio
    }
}

impl std::ops::Deref for VideoDecoder {
    type Target = GenericDecoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for VideoDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AudioDecoder {
    pub fn sample_rate(&self) -> i32 {
        self.0.decoder.as_deref_except().sample_rate
    }

    pub fn channels(&self) -> i32 {
        self.0.decoder.as_deref_except().ch_layout.nb_channels
    }

    pub fn sample_format(&self) -> AVSampleFormat {
        self.0.decoder.as_deref_except().sample_fmt
    }
}

impl std::ops::Deref for AudioDecoder {
    type Target = GenericDecoder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AudioDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use ffmpeg_sys_next::AVCodecID::{AV_CODEC_ID_AAC, AV_CODEC_ID_H264};
    use ffmpeg_sys_next::AVMediaType;

    use crate::codec::DecoderCodec;
    use crate::decoder::{Decoder, DecoderOptions};
    use crate::io::Input;

    #[test]
    fn test_generic_decoder_debug() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO)
                    .unwrap_or(false)
            })
            .expect("No video stream found");
        let codec_params = stream.codec_parameters().expect("Missing codec parameters");
        assert_eq!(
            codec_params.codec_type,
            AVMediaType::AVMEDIA_TYPE_VIDEO,
            "Expected the stream to be a video stream"
        );
        let decoder_options = DecoderOptions {
            codec: Some(DecoderCodec::new(AV_CODEC_ID_H264).expect("Failed to find H264 codec")),
            thread_count: 2,
        };
        let decoder = Decoder::with_options(&stream, decoder_options).expect("Failed to create Decoder");
        let generic_decoder = match decoder {
            Decoder::Video(video_decoder) => video_decoder.0,
            Decoder::Audio(audio_decoder) => audio_decoder.0,
        };

        insta::assert_debug_snapshot!(generic_decoder, @r"
        Decoder {
            time_base: AVRational {
                num: 1,
                den: 15360,
            },
            codec_type: AVMEDIA_TYPE_VIDEO,
        }
        ");
    }

    #[test]
    fn test_video_decoder_debug() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO)
                    .unwrap_or(false)
            })
            .expect("No video stream found");
        let codec_params = stream.codec_parameters().expect("Missing codec parameters");
        assert_eq!(
            codec_params.codec_type,
            AVMediaType::AVMEDIA_TYPE_VIDEO,
            "Expected the stream to be a video stream"
        );

        let decoder_options = DecoderOptions {
            codec: Some(DecoderCodec::new(AV_CODEC_ID_H264).expect("Failed to find H264 codec")),
            thread_count: 2,
        };
        let decoder = Decoder::with_options(&stream, decoder_options).expect("Failed to create Decoder");

        let generic_decoder = match decoder {
            Decoder::Video(video_decoder) => video_decoder,
            _ => panic!("Expected a video decoder, got something else"),
        };

        insta::assert_debug_snapshot!(generic_decoder, @r"
        VideoDecoder {
            time_base: AVRational {
                num: 1,
                den: 15360,
            },
            width: 3840,
            height: 2160,
            pixel_format: AV_PIX_FMT_YUV420P,
            frame_rate: AVRational {
                num: 60,
                den: 1,
            },
            sample_aspect_ratio: AVRational {
                num: 1,
                den: 1,
            },
        }
        ");
    }

    #[test]
    fn test_audio_decoder_debug() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_AUDIO)
                    .unwrap_or(false)
            })
            .expect("No audio stream found");
        let codec_params = stream.codec_parameters().expect("Missing codec parameters");
        assert_eq!(
            codec_params.codec_type,
            AVMediaType::AVMEDIA_TYPE_AUDIO,
            "Expected the stream to be an audio stream"
        );
        let decoder_options = DecoderOptions {
            codec: Some(DecoderCodec::new(AV_CODEC_ID_AAC).expect("Failed to find AAC codec")),
            thread_count: 2,
        };
        let decoder = Decoder::with_options(&stream, decoder_options).expect("Failed to create Decoder");
        let audio_decoder = match decoder {
            Decoder::Audio(audio_decoder) => audio_decoder,
            _ => panic!("Expected an audio decoder, got something else"),
        };

        insta::assert_debug_snapshot!(audio_decoder, @r"
        AudioDecoder {
            time_base: AVRational {
                num: 1,
                den: 48000,
            },
            sample_rate: 48000,
            channels: 2,
            sample_fmt: AV_SAMPLE_FMT_FLTP,
        }
        ");
    }

    #[test]
    fn test_decoder_options_default() {
        let default_options = DecoderOptions::default();

        assert!(default_options.codec.is_none(), "Expected default codec to be None");
        assert_eq!(default_options.thread_count, 1, "Expected default thread_count to be 1");
    }

    #[test]
    fn test_decoder_new() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO)
                    .unwrap_or(false)
            })
            .expect("No video stream found");

        let decoder_result = Decoder::new(&stream);
        assert!(decoder_result.is_ok(), "Expected Decoder::new to succeed, but it failed");

        let decoder = decoder_result.unwrap();
        if let Decoder::Video(video_decoder) = decoder {
            assert!(video_decoder.width() > 0, "Expected valid width for video stream");
            assert!(video_decoder.height() > 0, "Expected valid height for video stream");
        } else {
            panic!("Expected a video decoder, but got a different type");
        }
    }

    #[test]
    fn test_decoder_with_options_missing_codec_parameters() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");
        unsafe {
            (*stream.as_mut_ptr()).codecpar = std::ptr::null_mut();
        }
        let decoder_result = Decoder::with_options(&stream, DecoderOptions::default());

        assert!(decoder_result.is_err(), "Expected Decoder creation to fail");
        if let Err(err) = decoder_result {
            match err {
                crate::error::FfmpegError::NoDecoder => (),
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_decoder_with_options_non_video_audio_codec_type() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");
        unsafe {
            (*stream.as_mut_ptr()).codecpar.as_mut().unwrap().codec_type = AVMediaType::AVMEDIA_TYPE_SUBTITLE;
        }
        let decoder_result = Decoder::with_options(&stream, DecoderOptions::default());

        assert!(
            decoder_result.is_err(),
            "Expected Decoder creation to fail for non-video/audio codec type"
        );
        if let Err(err) = decoder_result {
            match err {
                crate::error::FfmpegError::NoDecoder => (),
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_video_decoder_deref_mut_safe() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO)
                    .unwrap_or(false)
            })
            .expect("No video stream found");
        let decoder_options = DecoderOptions {
            codec: None,
            thread_count: 2,
        };
        let decoder = Decoder::with_options(&stream, decoder_options).expect("Failed to create Decoder");
        let mut video_decoder = match decoder {
            Decoder::Video(video_decoder) => video_decoder,
            _ => panic!("Expected a VideoDecoder, got something else"),
        };
        {
            let generic_decoder = &mut *video_decoder;
            let mut time_base = generic_decoder.time_base();
            time_base.num = 1000;
            time_base.den = 1;
            generic_decoder.decoder.as_deref_mut_except().time_base = time_base;
        }
        let generic_decoder = &*video_decoder;
        let time_base = generic_decoder.decoder.as_deref_except().time_base;

        assert_eq!(time_base.num, 1000, "Expected time_base.num to be updated via DerefMut");
        assert_eq!(time_base.den, 1, "Expected time_base.den to be updated via DerefMut");
    }

    #[test]
    fn test_audio_decoder_deref_mut() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let stream = streams
            .iter()
            .find(|s| {
                s.codec_parameters()
                    .map(|p| p.codec_type == AVMediaType::AVMEDIA_TYPE_AUDIO)
                    .unwrap_or(false)
            })
            .expect("No audio stream found");
        let decoder_options = DecoderOptions {
            codec: None,
            thread_count: 2,
        };
        let decoder = Decoder::with_options(&stream, decoder_options).expect("Failed to create Decoder");
        let mut audio_decoder = match decoder {
            Decoder::Audio(audio_decoder) => audio_decoder,
            _ => panic!("Expected an AudioDecoder, got something else"),
        };
        {
            let generic_decoder = &mut *audio_decoder;
            let mut time_base = generic_decoder.time_base();
            time_base.num = 48000;
            time_base.den = 1;
            generic_decoder.decoder.as_deref_mut_except().time_base = time_base;
        }
        let generic_decoder = &*audio_decoder;
        let time_base = generic_decoder.decoder.as_deref_except().time_base;

        assert_eq!(time_base.num, 48000, "Expected time_base.num to be updated via DerefMut");
        assert_eq!(time_base.den, 1, "Expected time_base.den to be updated via DerefMut");
    }
}
