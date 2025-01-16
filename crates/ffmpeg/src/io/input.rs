use std::ffi::CStr;

use ffmpeg_sys_next::*;

use super::internal::{read_packet, seek, Inner, InnerOptions};
use crate::consts::{Const, DEFAULT_BUFFER_SIZE};
use crate::dict::Dictionary;
use crate::error::FfmpegError;
use crate::packet::{Packet, Packets};
use crate::smart_object::SmartObject;
use crate::stream::Streams;

pub struct Input<T: Send + Sync> {
    inner: SmartObject<Inner<T>>,
}

/// Safety: `Input` is safe to send between threads.
unsafe impl<T: Send + Sync> Send for Input<T> {}

#[derive(Debug, Clone)]
pub struct InputOptions<I: FnMut() -> bool> {
    pub buffer_size: usize,
    pub dictionary: Dictionary,
    pub interrupt_callback: Option<I>,
}

impl Default for InputOptions<fn() -> bool> {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            dictionary: Dictionary::new(),
            interrupt_callback: None,
        }
    }
}

impl<T: std::io::Read + Send + Sync> Input<T> {
    pub fn new(input: T) -> Result<Self, FfmpegError> {
        Self::with_options(input, &mut InputOptions::default())
    }

    pub fn with_options(input: T, options: &mut InputOptions<impl FnMut() -> bool>) -> Result<Self, FfmpegError> {
        Self::create_input(
            Inner::new(
                input,
                InnerOptions {
                    buffer_size: options.buffer_size,
                    read_fn: Some(read_packet::<T>),
                    ..Default::default()
                },
            )?,
            None,
            &mut options.dictionary,
        )
    }

    pub fn seekable(input: T) -> Result<Self, FfmpegError>
    where
        T: std::io::Seek,
    {
        Self::seekable_with_options(input, InputOptions::default())
    }

    pub fn seekable_with_options(input: T, mut options: InputOptions<impl FnMut() -> bool>) -> Result<Self, FfmpegError>
    where
        T: std::io::Seek,
    {
        Self::create_input(
            Inner::new(
                input,
                InnerOptions {
                    buffer_size: options.buffer_size,
                    read_fn: Some(read_packet::<T>),
                    seek_fn: Some(seek::<T>),
                    ..Default::default()
                },
            )?,
            None,
            &mut options.dictionary,
        )
    }
}

impl<T: Send + Sync> Input<T> {
    pub fn as_ptr(&self) -> *const AVFormatContext {
        self.inner.context.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.inner.context.as_mut_ptr()
    }

    pub fn streams(&self) -> Const<'_, Streams<'_>> {
        Const::new(Streams::new(self.inner.context.as_deref_except()))
    }

    pub fn packets(&mut self) -> Packets<'_> {
        Packets::new(self.inner.context.as_deref_mut_except())
    }

    pub fn receive_packet(&mut self) -> Result<Option<Packet>, FfmpegError> {
        self.packets().receive()
    }

    fn create_input(mut inner: Inner<T>, path: Option<&CStr>, dictionary: &mut Dictionary) -> Result<Self, FfmpegError> {
        // Safety: avformat_open_input is safe to call
        let ec = unsafe {
            avformat_open_input(
                inner.context.as_mut(),
                path.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()),
                std::ptr::null(),
                dictionary.as_mut_ptr_ref(),
            )
        };
        if ec != 0 {
            return Err(FfmpegError::Code(ec.into()));
        }

        if inner.context.as_ptr().is_null() {
            return Err(FfmpegError::Alloc);
        }

        let mut inner = SmartObject::new(inner, |inner| unsafe {
            // We own this resource so we need to free it
            avformat_close_input(inner.context.as_mut());
        });

        // We now own the context and this is freed when the object is dropped
        inner.context.set_destructor(|_| {});

        // Safety: avformat_find_stream_info is safe to call
        let ec = unsafe { avformat_find_stream_info(inner.context.as_mut_ptr(), std::ptr::null_mut()) };
        if ec < 0 {
            return Err(FfmpegError::Code(ec.into()));
        }

        Ok(Self { inner })
    }
}

impl Input<()> {
    pub fn open(path: &str) -> Result<Self, FfmpegError> {
        // We immediately create an input and setup the inner, before using it.
        let inner = unsafe { Inner::empty() };

        Self::create_input(inner, Some(&std::ffi::CString::new(path).unwrap()), &mut Dictionary::new())
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::io::Cursor;

    use super::{FfmpegError, Input, InputOptions, DEFAULT_BUFFER_SIZE};

    #[test]
    fn test_input_options_default() {
        let default_options = InputOptions::default();

        assert_eq!(default_options.buffer_size, DEFAULT_BUFFER_SIZE);
        assert!(default_options.dictionary.is_empty());
        assert!(default_options.interrupt_callback.is_none());
    }

    #[test]
    fn test_open_valid_file() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        assert!(std::path::Path::new(valid_file_path).exists(), "Test file does not exist");

        let result = Input::open(valid_file_path);
        assert!(result.is_ok(), "Expected success but got error");
    }

    #[test]
    fn test_open_invalid_path() {
        let invalid_path = "invalid_file.mp4";
        let result = Input::open(invalid_path);
        assert!(result.is_err(), "Expected an error for invalid path");
        if let Err(err) = result {
            match err {
                FfmpegError::Code(_) => (),
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }

    #[test]
    fn test_new_with_default_options() {
        let valid_media_data: Vec<u8> = include_bytes!("../../../../assets/avc_aac_large.mp4").to_vec();
        let data = Cursor::new(valid_media_data);
        let result = Input::new(data);

        if let Err(e) = &result {
            eprintln!("Error encountered: {:?}", e);
        }

        assert!(result.is_ok(), "Expected success but got error");
    }

    #[test]
    fn test_seekable_with_valid_input() {
        let valid_media_data: Vec<u8> = include_bytes!("../../../../assets/avc_aac_large.mp4").to_vec();
        let data = Cursor::new(valid_media_data);
        let result = Input::seekable(data);

        if let Err(e) = &result {
            eprintln!("Error encountered: {:?}", e);
        }

        assert!(result.is_ok(), "Expected success but got error");
    }

    #[test]
    fn test_as_ptr() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");

        let ptr = input.as_ptr();
        assert!(!ptr.is_null(), "Expected non-null pointer");
    }

    #[test]
    fn test_as_mut_ptr() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");

        let ptr = input.as_mut_ptr();
        assert!(!ptr.is_null(), "Expected non-null mutable pointer");
    }

    #[test]
    fn test_streams() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();

        assert!(!streams.is_empty(), "Expected at least one stream");

        let mut settings = insta::Settings::new();
        settings.add_filter(r"av_class: 0x[0-9a-f]+", "av_class: [Pointer]");
        settings.add_filter(r"iformat: 0x[0-9a-f]+", "iformat: [Pointer]");
        settings.add_filter(r"priv_data: 0x[0-9a-f]+", "priv_data: [Pointer]");
        settings.add_filter(r"pb: 0x[0-9a-f]+", "pb: [Pointer]");
        settings.add_filter(r"streams: 0x[0-9a-f]+", "streams: [Pointer]");
        settings.add_filter(r"url: 0x[0-9a-f]+", "url: [Pointer]");
        settings.add_filter(r"metadata: 0x[0-9a-f]+", "metadata: [Pointer]");
        settings.add_filter(r"protocol_whitelist: 0x[0-9a-f]+", "protocol_whitelist: [Pointer]");
        settings.add_filter(r"dump_separator: 0x[0-9a-f]+", "dump_separator: [Pointer]");
        settings.add_filter(r"io_open:\s*Some\(\s*0x[0-9a-f]+,\s*\)", "io_open: Some([Pointer])");
        settings.add_filter(r"io_close2:\s*Some\(\s*0x[0-9a-f]+,\s*\)", "io_close2: Some([Pointer])");

        settings.bind(|| {
            insta::assert_debug_snapshot!(streams, @r"
            Streams {
                input: AVFormatContext {
                    av_class: [Pointer],
                    iformat: [Pointer],
                    oformat: 0x0000000000000000,
                    priv_data: [Pointer],
                    pb: [Pointer],
                    ctx_flags: 0,
                    nb_streams: 2,
                    streams: [Pointer],
                    nb_stream_groups: 0,
                    stream_groups: 0x0000000000000000,
                    nb_chapters: 0,
                    chapters: 0x0000000000000000,
                    url: [Pointer],
                    start_time: 0,
                    duration: 1066667,
                    bit_rate: 1900416,
                    packet_size: 0,
                    max_delay: -1,
                    flags: 2097152,
                    probesize: 5000000,
                    max_analyze_duration: 0,
                    key: 0x0000000000000000,
                    keylen: 0,
                    nb_programs: 0,
                    programs: 0x0000000000000000,
                    video_codec_id: AV_CODEC_ID_NONE,
                    audio_codec_id: AV_CODEC_ID_NONE,
                    subtitle_codec_id: AV_CODEC_ID_NONE,
                    data_codec_id: AV_CODEC_ID_NONE,
                    metadata: [Pointer],
                    start_time_realtime: -9223372036854775808,
                    fps_probe_size: -1,
                    error_recognition: 1,
                    interrupt_callback: AVIOInterruptCB {
                        callback: None,
                        opaque: 0x0000000000000000,
                    },
                    debug: 0,
                    max_streams: 1000,
                    max_index_size: 1048576,
                    max_picture_buffer: 3041280,
                    max_interleave_delta: 10000000,
                    max_ts_probe: 50,
                    max_chunk_duration: 0,
                    max_chunk_size: 0,
                    max_probe_packets: 2500,
                    strict_std_compliance: 0,
                    event_flags: 1,
                    avoid_negative_ts: -1,
                    audio_preload: 0,
                    use_wallclock_as_timestamps: 0,
                    skip_estimate_duration_from_pts: 0,
                    avio_flags: 0,
                    duration_estimation_method: AVFMT_DURATION_FROM_STREAM,
                    skip_initial_bytes: 0,
                    correct_ts_overflow: 1,
                    seek2any: 0,
                    flush_packets: -1,
                    probe_score: 100,
                    format_probesize: 1048576,
                    codec_whitelist: 0x0000000000000000,
                    format_whitelist: 0x0000000000000000,
                    protocol_whitelist: [Pointer],
                    protocol_blacklist: 0x0000000000000000,
                    io_repositioned: 0,
                    video_codec: 0x0000000000000000,
                    audio_codec: 0x0000000000000000,
                    subtitle_codec: 0x0000000000000000,
                    data_codec: 0x0000000000000000,
                    metadata_header_padding: -1,
                    opaque: 0x0000000000000000,
                    control_message_cb: None,
                    output_ts_offset: 0,
                    dump_separator: [Pointer],
                    io_open: Some([Pointer]),
                    io_close2: Some([Pointer]),
                    duration_probesize: 0,
                },
            }
            ");
        });
    }

    #[test]
    fn test_packets() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut packets = input.packets();

        for _ in 0..5 {
            match packets.next() {
                Some(Ok(_)) => (),
                Some(Err(e)) => panic!("Error encountered while reading packets: {:?}", e),
                None => break,
            }
        }

        let mut settings = insta::Settings::new();
        settings.add_filter(r"av_class: 0x[0-9a-f]+", "av_class: [Pointer]");
        settings.add_filter(r"iformat: 0x[0-9a-f]+", "iformat: [Pointer]");
        settings.add_filter(r"priv_data: 0x[0-9a-f]+", "priv_data: [Pointer]");
        settings.add_filter(r"pb: 0x[0-9a-f]+", "pb: [Pointer]");
        settings.add_filter(r"streams: 0x[0-9a-f]+", "streams: [Pointer]");
        settings.add_filter(r"url: 0x[0-9a-f]+", "url: [Pointer]");
        settings.add_filter(r"metadata: 0x[0-9a-f]+", "metadata: [Pointer]");
        settings.add_filter(r"protocol_whitelist: 0x[0-9a-f]+", "protocol_whitelist: [Pointer]");
        settings.add_filter(r"dump_separator: 0x[0-9a-f]+", "dump_separator: [Pointer]");
        settings.add_filter(r"io_open:\s*Some\(\s*0x[0-9a-f]+,\s*\)", "io_open: Some([Pointer])");
        settings.add_filter(r"io_close2:\s*Some\(\s*0x[0-9a-f]+,\s*\)", "io_close2: Some([Pointer])");

        settings.bind(|| {
            insta::assert_debug_snapshot!(packets, @r"
            Packets {
                context: AVFormatContext {
                    av_class: [Pointer],
                    iformat: [Pointer],
                    oformat: 0x0000000000000000,
                    priv_data: [Pointer],
                    pb: [Pointer],
                    ctx_flags: 0,
                    nb_streams: 2,
                    streams: [Pointer],
                    nb_stream_groups: 0,
                    stream_groups: 0x0000000000000000,
                    nb_chapters: 0,
                    chapters: 0x0000000000000000,
                    url: [Pointer],
                    start_time: 0,
                    duration: 1066667,
                    bit_rate: 1900416,
                    packet_size: 0,
                    max_delay: -1,
                    flags: 2097152,
                    probesize: 5000000,
                    max_analyze_duration: 0,
                    key: 0x0000000000000000,
                    keylen: 0,
                    nb_programs: 0,
                    programs: 0x0000000000000000,
                    video_codec_id: AV_CODEC_ID_NONE,
                    audio_codec_id: AV_CODEC_ID_NONE,
                    subtitle_codec_id: AV_CODEC_ID_NONE,
                    data_codec_id: AV_CODEC_ID_NONE,
                    metadata: [Pointer],
                    start_time_realtime: -9223372036854775808,
                    fps_probe_size: -1,
                    error_recognition: 1,
                    interrupt_callback: AVIOInterruptCB {
                        callback: None,
                        opaque: 0x0000000000000000,
                    },
                    debug: 0,
                    max_streams: 1000,
                    max_index_size: 1048576,
                    max_picture_buffer: 3041280,
                    max_interleave_delta: 10000000,
                    max_ts_probe: 50,
                    max_chunk_duration: 0,
                    max_chunk_size: 0,
                    max_probe_packets: 2500,
                    strict_std_compliance: 0,
                    event_flags: 1,
                    avoid_negative_ts: -1,
                    audio_preload: 0,
                    use_wallclock_as_timestamps: 0,
                    skip_estimate_duration_from_pts: 0,
                    avio_flags: 0,
                    duration_estimation_method: AVFMT_DURATION_FROM_STREAM,
                    skip_initial_bytes: 0,
                    correct_ts_overflow: 1,
                    seek2any: 0,
                    flush_packets: -1,
                    probe_score: 100,
                    format_probesize: 1048576,
                    codec_whitelist: 0x0000000000000000,
                    format_whitelist: 0x0000000000000000,
                    protocol_whitelist: [Pointer],
                    protocol_blacklist: 0x0000000000000000,
                    io_repositioned: 0,
                    video_codec: 0x0000000000000000,
                    audio_codec: 0x0000000000000000,
                    subtitle_codec: 0x0000000000000000,
                    data_codec: 0x0000000000000000,
                    metadata_header_padding: -1,
                    opaque: 0x0000000000000000,
                    control_message_cb: None,
                    output_ts_offset: 0,
                    dump_separator: [Pointer],
                    io_open: Some([Pointer]),
                    io_close2: Some([Pointer]),
                    duration_probesize: 0,
                },
            }
            ");
        });
    }

    #[test]
    fn test_receive_packet() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");

        match input.receive_packet() {
            Ok(Some(packet)) => {
                assert!(!packet.data().is_empty(), "Expected a non-empty packet");
                assert!(packet.stream_index() >= 0, "Expected a valid stream index");
                insta::assert_debug_snapshot!(packet, @r"
                Packet {
                    stream_index: 0,
                    pts: Some(
                        0,
                    ),
                    dts: Some(
                        -512,
                    ),
                    duration: Some(
                        256,
                    ),
                    pos: Some(
                        48,
                    ),
                    is_key: true,
                    is_corrupt: false,
                    is_discard: false,
                    is_trusted: false,
                    is_disposable: false,
                }
                ");
            }
            Ok(None) => panic!("Expected a packet but received None"),
            Err(e) => panic!("Error encountered while receiving packet: {:?}", e),
        }
    }
}
