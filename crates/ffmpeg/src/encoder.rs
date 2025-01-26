use ffmpeg_sys_next::*;

use crate::codec::EncoderCodec;
use crate::dict::Dictionary;
use crate::error::FfmpegError;
use crate::frame::Frame;
use crate::io::Output;
use crate::packet::Packet;
use crate::smart_object::{SmartObject, SmartPtr};

pub struct Encoder {
    incoming_time_base: AVRational,
    outgoing_time_base: AVRational,
    encoder: SmartPtr<AVCodecContext>,
    stream_index: i32,
    average_duration: i64,
    previous_dts: i64,
}

/// Safety: `Encoder` can be sent between threads.
unsafe impl Send for Encoder {}

pub struct VideoEncoderSettings {
    width: i32,
    height: i32,
    frame_rate: i32,
    pixel_format: AVPixelFormat,
    gop_size: Option<i32>,
    qmax: Option<i32>,
    qmin: Option<i32>,
    thread_count: Option<i32>,
    thread_type: Option<i32>,
    sample_aspect_ratio: Option<AVRational>,
    bitrate: Option<i64>,
    rc_min_rate: Option<i64>,
    rc_max_rate: Option<i64>,
    rc_buffer_size: Option<i32>,
    max_b_frames: Option<i32>,
    codec_specific_options: Option<Dictionary>,
    flags: Option<i32>,
    flags2: Option<i32>,
}

impl Default for VideoEncoderSettings {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            frame_rate: 0,
            pixel_format: AVPixelFormat::AV_PIX_FMT_NONE,
            gop_size: None,
            qmax: None,
            qmin: None,
            thread_count: None,
            thread_type: None,
            sample_aspect_ratio: None,
            bitrate: None,
            rc_min_rate: None,
            rc_max_rate: None,
            rc_buffer_size: None,
            max_b_frames: None,
            codec_specific_options: None,
            flags: None,
            flags2: None,
        }
    }
}

impl VideoEncoderSettings {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            frame_rate: 0,
            pixel_format: AVPixelFormat::AV_PIX_FMT_NONE,
            gop_size: None,
            qmax: None,
            qmin: None,
            thread_count: None,
            thread_type: None,
            sample_aspect_ratio: None,
            bitrate: None,
            rc_min_rate: None,
            rc_max_rate: None,
            rc_buffer_size: None,
            max_b_frames: None,
            codec_specific_options: None,
            flags: None,
            flags2: None,
        }
    }

    fn apply(self, encoder: &mut AVCodecContext) -> Result<(), FfmpegError> {
        if self.width <= 0 || self.height <= 0 || self.frame_rate <= 0 || self.pixel_format == AVPixelFormat::AV_PIX_FMT_NONE
        {
            return Err(FfmpegError::Arguments(
                "width, height, frame_rate and pixel_format must be set",
            ));
        }

        encoder.width = self.width;
        encoder.height = self.height;

        encoder.pix_fmt = self.pixel_format;
        encoder.sample_aspect_ratio = self.sample_aspect_ratio.unwrap_or(encoder.sample_aspect_ratio);
        encoder.framerate = AVRational {
            num: self.frame_rate,
            den: 1,
        };
        encoder.thread_count = self.thread_count.unwrap_or(encoder.thread_count);
        encoder.thread_type = self.thread_type.unwrap_or(encoder.thread_type);
        encoder.gop_size = self.gop_size.unwrap_or(encoder.gop_size);
        encoder.qmax = self.qmax.unwrap_or(encoder.qmax);
        encoder.qmin = self.qmin.unwrap_or(encoder.qmin);
        encoder.bit_rate = self.bitrate.unwrap_or(encoder.bit_rate);
        encoder.rc_min_rate = self.rc_min_rate.unwrap_or(encoder.rc_min_rate);
        encoder.rc_max_rate = self.rc_max_rate.unwrap_or(encoder.rc_max_rate);
        encoder.rc_buffer_size = self.rc_buffer_size.unwrap_or(encoder.rc_buffer_size);
        encoder.max_b_frames = self.max_b_frames.unwrap_or(encoder.max_b_frames);
        encoder.flags = self.flags.unwrap_or(encoder.flags);
        encoder.flags2 = self.flags2.unwrap_or(encoder.flags2);

        Ok(())
    }

    fn average_duration(&self, timebase: AVRational) -> i64 {
        if self.frame_rate <= 0 {
            return 0;
        }

        (timebase.den as i64) / (self.frame_rate as i64 * timebase.num as i64)
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn frame_rate(&self) -> i32 {
        self.frame_rate
    }

    pub fn set_frame_rate(&mut self, frame_rate: i32) {
        self.frame_rate = frame_rate;
    }

    pub fn pixel_format(&self) -> AVPixelFormat {
        self.pixel_format
    }

    pub fn set_pixel_format(&mut self, pixel_format: AVPixelFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn gop_size(&self) -> Option<i32> {
        self.gop_size
    }

    pub fn set_gop_size(&mut self, gop_size: i32) {
        self.gop_size = Some(gop_size);
    }

    pub fn qmax(&self) -> Option<i32> {
        self.qmax
    }

    pub fn set_qmax(&mut self, qmax: i32) {
        self.qmax = Some(qmax);
    }

    pub fn qmin(&self) -> Option<i32> {
        self.qmin
    }

    pub fn set_qmin(&mut self, qmin: i32) {
        self.qmin = Some(qmin);
    }

    pub fn thread_count(&self) -> Option<i32> {
        self.thread_count
    }

    pub fn set_thread_count(&mut self, thread_count: i32) {
        self.thread_count = Some(thread_count);
    }

    pub fn thread_type(&self) -> Option<i32> {
        self.thread_type
    }

    pub fn set_thread_type(&mut self, thread_type: i32) {
        self.thread_type = Some(thread_type);
    }

    pub fn sample_aspect_ratio(&self) -> Option<AVRational> {
        self.sample_aspect_ratio
    }

    pub fn set_sample_aspect_ratio(&mut self, sample_aspect_ratio: AVRational) {
        self.sample_aspect_ratio = Some(sample_aspect_ratio);
    }

    pub fn bitrate(&self) -> Option<i64> {
        self.bitrate
    }

    pub fn set_bitrate(&mut self, bitrate: i64) {
        self.bitrate = Some(bitrate);
    }

    pub fn rc_min_rate(&self) -> Option<i64> {
        self.rc_min_rate
    }

    pub fn set_rc_min_rate(&mut self, rc_min_rate: i64) {
        self.rc_min_rate = Some(rc_min_rate);
    }

    pub fn rc_max_rate(&self) -> Option<i64> {
        self.rc_max_rate
    }

    pub fn set_rc_max_rate(&mut self, rc_max_rate: i64) {
        self.rc_max_rate = Some(rc_max_rate);
    }

    pub fn rc_buffer_size(&self) -> Option<i32> {
        self.rc_buffer_size
    }

    pub fn set_rc_buffer_size(&mut self, rc_buffer_size: i32) {
        self.rc_buffer_size = Some(rc_buffer_size);
    }

    pub fn max_b_frames(&self) -> Option<i32> {
        self.max_b_frames
    }

    pub fn set_max_b_frames(&mut self, max_b_frames: i32) {
        self.max_b_frames = Some(max_b_frames);
    }

    pub fn codec_specific_options(&self) -> Option<&Dictionary> {
        self.codec_specific_options.as_ref()
    }

    pub fn set_codec_specific_options(&mut self, codec_specific_options: Dictionary) {
        self.codec_specific_options = Some(codec_specific_options);
    }

    pub fn flags(&self) -> Option<i32> {
        self.flags
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = Some(flags);
    }

    pub fn flags2(&self) -> Option<i32> {
        self.flags2
    }

    pub fn set_flags2(&mut self, flags2: i32) {
        self.flags2 = Some(flags2);
    }
}

pub struct AudioEncoderSettings {
    sample_rate: i32,
    ch_layout: Option<SmartObject<AVChannelLayout>>,
    sample_fmt: AVSampleFormat,
    thread_count: Option<i32>,
    thread_type: Option<i32>,
    bitrate: Option<i64>,
    buffer_size: Option<i64>,
    rc_min_rate: Option<i64>,
    rc_max_rate: Option<i64>,
    rc_buffer_size: Option<i32>,
    codec_specific_options: Option<Dictionary>,
    flags: Option<i32>,
    flags2: Option<i32>,
}

impl Default for AudioEncoderSettings {
    fn default() -> Self {
        Self {
            sample_rate: 0,
            ch_layout: None,
            sample_fmt: AVSampleFormat::AV_SAMPLE_FMT_NONE,
            thread_count: None,
            thread_type: None,
            bitrate: None,
            buffer_size: None,
            rc_min_rate: None,
            rc_max_rate: None,
            rc_buffer_size: None,
            codec_specific_options: None,
            flags: None,
            flags2: None,
        }
    }
}

impl AudioEncoderSettings {
    pub fn new() -> Self {
        Self {
            sample_rate: 0,
            ch_layout: None,
            sample_fmt: AVSampleFormat::AV_SAMPLE_FMT_NONE,
            thread_count: None,
            thread_type: None,
            bitrate: None,
            buffer_size: None,
            rc_min_rate: None,
            rc_max_rate: None,
            rc_buffer_size: None,
            codec_specific_options: None,
            flags: None,
            flags2: None,
        }
    }

    fn apply(self, encoder: &mut AVCodecContext) -> Result<(), FfmpegError> {
        if self.sample_rate <= 0 || self.ch_layout.is_none() || self.sample_fmt == AVSampleFormat::AV_SAMPLE_FMT_NONE {
            return Err(FfmpegError::Arguments(
                "sample_rate, channel_layout and sample_fmt must be set",
            ));
        }

        encoder.sample_rate = self.sample_rate;
        encoder.ch_layout = self.ch_layout.unwrap().into_inner();
        encoder.sample_fmt = self.sample_fmt;
        encoder.thread_count = self.thread_count.unwrap_or(encoder.thread_count);
        encoder.thread_type = self.thread_type.unwrap_or(encoder.thread_type);
        encoder.bit_rate = self.bitrate.unwrap_or(encoder.bit_rate);
        encoder.rc_min_rate = self.rc_min_rate.unwrap_or(encoder.rc_min_rate);
        encoder.rc_max_rate = self.rc_max_rate.unwrap_or(encoder.rc_max_rate);
        encoder.rc_buffer_size = self.rc_buffer_size.unwrap_or(encoder.rc_buffer_size);
        encoder.flags = self.flags.unwrap_or(encoder.flags);
        encoder.flags2 = self.flags2.unwrap_or(encoder.flags2);

        Ok(())
    }

    fn average_duration(&self, timebase: AVRational) -> i64 {
        if self.sample_rate <= 0 {
            return 0;
        }

        (timebase.den as i64) / (self.sample_rate as i64 * timebase.num as i64)
    }

    pub fn sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn set_sample_rate(&mut self, sample_rate: i32) {
        self.sample_rate = sample_rate;
    }

    pub fn channel_count(&self) -> i32 {
        self.ch_layout.as_ref().map(|ch_layout| ch_layout.nb_channels).unwrap_or(0)
    }

    pub fn set_channel_count(&mut self, channel_count: i32) {
        let mut ch_layout = SmartObject::new(unsafe { std::mem::zeroed() }, |ptr| unsafe { av_channel_layout_uninit(ptr) });
        unsafe { av_channel_layout_default(ch_layout.as_mut(), channel_count) };
        self.ch_layout = Some(ch_layout);
    }

    pub fn ch_layout(&self) -> Option<SmartObject<AVChannelLayout>> {
        self.ch_layout.clone()
    }

    pub fn set_ch_layout(&mut self, custom_layout: AVChannelLayout) -> Result<(), FfmpegError> {
        let smart_object = SmartObject::new(custom_layout, |ptr| unsafe { ffmpeg_sys_next::av_channel_layout_uninit(ptr) });

        unsafe {
            if ffmpeg_sys_next::av_channel_layout_check(&*smart_object) == 0 {
                return Err(FfmpegError::Arguments("Invalid channel layout."));
            }
        }

        self.ch_layout = Some(smart_object);
        Ok(())
    }

    pub fn sample_fmt(&self) -> AVSampleFormat {
        self.sample_fmt
    }

    pub fn set_sample_fmt(&mut self, sample_fmt: AVSampleFormat) {
        self.sample_fmt = sample_fmt;
    }

    pub fn thread_count(&self) -> Option<i32> {
        self.thread_count
    }

    pub fn set_thread_count(&mut self, thread_count: i32) {
        self.thread_count = Some(thread_count);
    }

    pub fn thread_type(&self) -> Option<i32> {
        self.thread_type
    }

    pub fn set_thread_type(&mut self, thread_type: i32) {
        self.thread_type = Some(thread_type);
    }

    pub fn bitrate(&self) -> Option<i64> {
        self.bitrate
    }

    pub fn set_bitrate(&mut self, bitrate: i64) {
        self.bitrate = Some(bitrate);
    }

    pub fn buffer_size(&self) -> Option<i64> {
        self.buffer_size
    }

    pub fn set_buffer_size(&mut self, buffer_size: i64) {
        self.buffer_size = Some(buffer_size);
    }

    pub fn rc_min_rate(&self) -> Option<i64> {
        self.rc_min_rate
    }

    pub fn set_rc_min_rate(&mut self, rc_min_rate: i64) {
        self.rc_min_rate = Some(rc_min_rate);
    }

    pub fn rc_max_rate(&self) -> Option<i64> {
        self.rc_max_rate
    }

    pub fn set_rc_max_rate(&mut self, rc_max_rate: i64) {
        self.rc_max_rate = Some(rc_max_rate);
    }

    pub fn rc_buffer_size(&self) -> Option<i32> {
        self.rc_buffer_size
    }

    pub fn set_rc_buffer_size(&mut self, rc_buffer_size: i32) {
        self.rc_buffer_size = Some(rc_buffer_size);
    }

    pub fn codec_specific_options(&self) -> Option<&Dictionary> {
        self.codec_specific_options.as_ref()
    }

    pub fn set_codec_specific_options(&mut self, codec_specific_options: Dictionary) {
        self.codec_specific_options = Some(codec_specific_options);
    }

    pub fn flags(&self) -> Option<i32> {
        self.flags
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = Some(flags);
    }

    pub fn flags2(&self) -> Option<i32> {
        self.flags2
    }

    pub fn set_flags2(&mut self, flags2: i32) {
        self.flags2 = Some(flags2);
    }
}

pub enum EncoderSettings {
    Video(VideoEncoderSettings),
    Audio(AudioEncoderSettings),
}

impl EncoderSettings {
    fn apply(self, encoder: &mut AVCodecContext) -> Result<(), FfmpegError> {
        match self {
            EncoderSettings::Video(video_settings) => video_settings.apply(encoder),
            EncoderSettings::Audio(audio_settings) => audio_settings.apply(encoder),
        }
    }

    fn codec_specific_options(&mut self) -> Option<&mut Dictionary> {
        match self {
            EncoderSettings::Video(video_settings) => video_settings.codec_specific_options.as_mut(),
            EncoderSettings::Audio(audio_settings) => audio_settings.codec_specific_options.as_mut(),
        }
    }

    fn average_duration(&self, timebase: AVRational) -> i64 {
        match self {
            EncoderSettings::Video(video_settings) => video_settings.average_duration(timebase),
            EncoderSettings::Audio(audio_settings) => audio_settings.average_duration(timebase),
        }
    }
}

impl From<VideoEncoderSettings> for EncoderSettings {
    fn from(settings: VideoEncoderSettings) -> Self {
        EncoderSettings::Video(settings)
    }
}

impl From<AudioEncoderSettings> for EncoderSettings {
    fn from(settings: AudioEncoderSettings) -> Self {
        EncoderSettings::Audio(settings)
    }
}

impl Encoder {
    fn new<T: Send + Sync>(
        codec: EncoderCodec,
        output: &mut Output<T>,
        incoming_time_base: AVRational,
        outgoing_time_base: AVRational,
        settings: impl Into<EncoderSettings>,
    ) -> Result<Self, FfmpegError> {
        if codec.as_ptr().is_null() {
            return Err(FfmpegError::NoEncoder);
        }

        let mut settings = settings.into();

        let global_header = output.flags() & AVFMT_GLOBALHEADER != 0;

        // Safety: `avcodec_alloc_context3` is safe to call, and the pointer returned is
        // valid.
        let mut encoder =
            unsafe { SmartPtr::wrap_non_null(avcodec_alloc_context3(codec.as_ptr()), |ptr| avcodec_free_context(ptr)) }
                .ok_or(FfmpegError::Alloc)?;

        let mut ost = output.add_stream(None).ok_or(FfmpegError::NoStream)?;

        let encoder_mut = encoder.as_deref_mut_except();

        encoder_mut.time_base = incoming_time_base;

        let mut codec_options = settings.codec_specific_options().cloned();

        let codec_options_ptr = codec_options
            .as_mut()
            .map(|options| options.as_mut_ptr_ref() as *mut *mut _)
            .unwrap_or(std::ptr::null_mut());

        let average_duration = settings.average_duration(outgoing_time_base);

        settings.apply(encoder_mut)?;

        if global_header {
            encoder_mut.flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
        }

        // Safety: `avcodec_open2` is safe to call, 'encoder' and 'codec' and
        // 'codec_options_ptr' are a valid pointers.
        let res = unsafe { avcodec_open2(encoder_mut, codec.as_ptr(), codec_options_ptr) };
        if res < 0 {
            return Err(FfmpegError::Code(res.into()));
        }

        // Safety: `avcodec_parameters_from_context` is safe to call, 'ost' and
        // 'encoder' are valid pointers.
        let ret = unsafe { avcodec_parameters_from_context((*ost.as_mut_ptr()).codecpar, encoder_mut) };
        if ret < 0 {
            return Err(FfmpegError::Code(ret.into()));
        }

        ost.set_time_base(outgoing_time_base);

        Ok(Self {
            incoming_time_base,
            outgoing_time_base,
            encoder,
            average_duration,
            stream_index: ost.index(),
            previous_dts: 0,
        })
    }

    pub fn send_eof(&mut self) -> Result<(), FfmpegError> {
        // Safety: `self.encoder` is a valid pointer.
        let ret = unsafe { avcodec_send_frame(self.encoder.as_mut_ptr(), std::ptr::null()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(FfmpegError::Code(ret.into()))
        }
    }

    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), FfmpegError> {
        // Safety: `self.encoder` and `frame` are valid pointers.
        let ret = unsafe { avcodec_send_frame(self.encoder.as_mut_ptr(), frame.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(FfmpegError::Code(ret.into()))
        }
    }

    pub fn receive_packet(&mut self) -> Result<Option<Packet>, FfmpegError> {
        let mut packet = Packet::new()?;

        const AVERROR_EAGAIN: i32 = AVERROR(EAGAIN);

        // Safety: `self.encoder` and `packet` are valid pointers.
        let ret = unsafe { avcodec_receive_packet(self.encoder.as_mut_ptr(), packet.as_mut_ptr()) };

        match ret {
            AVERROR_EAGAIN | AVERROR_EOF => Ok(None),
            0 => {
                assert!(packet.dts().is_some(), "packet dts is none");
                let packet_dts = packet.dts().unwrap();
                assert!(
                    packet_dts >= self.previous_dts,
                    "packet dts is less than previous dts: {} >= {}",
                    packet_dts,
                    self.previous_dts
                );
                self.previous_dts = packet_dts;
                packet.rescale_timebase(self.incoming_time_base, self.outgoing_time_base);
                packet.set_stream_index(self.stream_index);
                Ok(Some(packet))
            }
            _ => Err(FfmpegError::Code(ret.into())),
        }
    }

    pub fn stream_index(&self) -> i32 {
        self.stream_index
    }

    pub fn incoming_time_base(&self) -> AVRational {
        self.incoming_time_base
    }

    pub fn outgoing_time_base(&self) -> AVRational {
        self.outgoing_time_base
    }
}

pub struct MuxerEncoder<T: Send + Sync> {
    encoder: Encoder,
    output: Output<T>,
    interleave: bool,
    muxer_headers_written: bool,
    muxer_options: Dictionary,
    buffered_packet: Option<Packet>,
    previous_dts: i64,
    previous_pts: i64,
}

#[derive(Clone, Debug)]
pub struct MuxerSettings {
    pub interleave: bool,
    pub muxer_options: Dictionary,
}

impl Default for MuxerSettings {
    fn default() -> Self {
        Self {
            interleave: true,
            muxer_options: Dictionary::new(),
        }
    }
}

impl MuxerSettings {
    pub fn builder() -> MuxerSettingsBuilder {
        MuxerSettingsBuilder::default()
    }
}

#[derive(Clone, Default, Debug)]
pub struct MuxerSettingsBuilder(MuxerSettings);

impl MuxerSettingsBuilder {
    pub fn interleave(mut self, interleave: bool) -> Self {
        self.0.interleave = interleave;
        self
    }

    pub fn muxer_options(mut self, muxer_options: Dictionary) -> Self {
        self.0.muxer_options = muxer_options;
        self
    }

    pub fn build(self) -> MuxerSettings {
        self.0
    }
}

impl<T: Send + Sync> MuxerEncoder<T> {
    pub fn new(
        codec: EncoderCodec,
        mut output: Output<T>,
        incoming_time_base: AVRational,
        outgoing_time_base: AVRational,
        settings: impl Into<EncoderSettings>,
        muxer_settings: MuxerSettings,
    ) -> Result<Self, FfmpegError> {
        Ok(Self {
            encoder: Encoder::new(codec, &mut output, incoming_time_base, outgoing_time_base, settings)?,
            output,
            interleave: muxer_settings.interleave,
            muxer_options: muxer_settings.muxer_options,
            muxer_headers_written: false,
            previous_dts: -1,
            previous_pts: -1,
            buffered_packet: None,
        })
    }

    pub fn send_eof(&mut self) -> Result<(), FfmpegError> {
        self.encoder.send_eof()?;
        self.handle_packets()?;

        if let Some(mut bufferd_packet) = self.buffered_packet.take() {
            if let Some(dts) = bufferd_packet.dts() {
                if dts == self.previous_dts {
                    bufferd_packet.set_dts(Some(dts + 1));
                }

                self.previous_dts = dts;
            }

            if let Some(pts) = bufferd_packet.pts() {
                if pts == self.previous_pts {
                    bufferd_packet.set_pts(Some(pts + 1));
                }

                self.previous_pts = pts;
            }

            bufferd_packet.set_duration(Some(self.average_duration));

            if self.interleave {
                self.output.write_interleaved_packet(bufferd_packet)?;
            } else {
                self.output.write_packet(&bufferd_packet)?;
            }
        }

        if !self.muxer_headers_written {
            self.output.write_header_with_options(&mut self.muxer_options)?;
            self.muxer_headers_written = true;
        }

        self.output.write_trailer()?;
        Ok(())
    }

    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), FfmpegError> {
        self.encoder.send_frame(frame)?;
        self.handle_packets()?;
        Ok(())
    }

    pub fn handle_packets(&mut self) -> Result<(), FfmpegError> {
        while let Some(packet) = self.encoder.receive_packet()? {
            if !self.muxer_headers_written {
                self.output.write_header_with_options(&mut self.muxer_options)?;
                self.muxer_headers_written = true;
            }

            if let Some(mut bufferd_packet) = self.buffered_packet.take() {
                if bufferd_packet.duration().unwrap_or(0) == 0 {
                    match ((packet.dts(), bufferd_packet.dts()), (packet.pts(), bufferd_packet.pts())) {
                        ((Some(packet_dts), Some(bufferd_dts)), _) if bufferd_dts < packet_dts => {
                            bufferd_packet.set_duration(Some(packet_dts - bufferd_dts))
                        }
                        (_, (Some(packet_pts), Some(bufferd_pts))) if bufferd_pts < packet_pts => {
                            bufferd_packet.set_duration(Some(packet_pts - bufferd_pts))
                        }
                        _ => bufferd_packet.set_duration(Some(self.encoder.average_duration)),
                    }
                }

                if let Some(dts) = bufferd_packet.dts() {
                    if dts == self.previous_dts {
                        bufferd_packet.set_dts(Some(dts + 1));
                    }

                    self.previous_dts = dts;
                }

                if let Some(pts) = bufferd_packet.pts() {
                    if pts == self.previous_pts {
                        bufferd_packet.set_pts(Some(pts + 1));
                    }

                    self.previous_pts = pts;
                }

                if self.interleave {
                    self.output.write_interleaved_packet(bufferd_packet)?;
                } else {
                    self.output.write_packet(&bufferd_packet)?;
                }
            }

            self.buffered_packet = Some(packet);
        }

        Ok(())
    }

    pub fn stream_index(&self) -> i32 {
        self.encoder.stream_index()
    }

    pub fn incoming_time_base(&self) -> AVRational {
        self.encoder.incoming_time_base()
    }

    pub fn outgoing_time_base(&self) -> AVRational {
        self.encoder.outgoing_time_base()
    }

    pub fn into_inner(self) -> Output<T> {
        self.output
    }
}

impl<T: Send + Sync> std::ops::Deref for MuxerEncoder<T> {
    type Target = Encoder;

    fn deref(&self) -> &Self::Target {
        &self.encoder
    }
}

impl<T: Send + Sync> std::ops::DerefMut for MuxerEncoder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.encoder
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use ffmpeg_sys_next::AVCodecID::AV_CODEC_ID_MPEG4;
    use ffmpeg_sys_next::{
        av_channel_layout_uninit, AVChannelLayout, AVChannelOrder, AVCodecContext, AVPixelFormat, AVRational,
        AVSampleFormat, AV_CH_LAYOUT_STEREO,
    };

    use crate::codec::EncoderCodec;
    use crate::dict::Dictionary;
    use crate::encoder::{
        AudioEncoderSettings, Encoder, EncoderSettings, MuxerEncoder, MuxerSettings, VideoEncoderSettings,
    };
    use crate::error::FfmpegError;
    use crate::io::{Output, OutputOptions};
    use crate::smart_object::SmartObject;

    #[test]
    fn test_video_encoder_settings_default() {
        let default_settings = VideoEncoderSettings::default();
        assert_eq!(default_settings.width(), 0);
        assert_eq!(default_settings.height(), 0);
        assert_eq!(default_settings.frame_rate(), 0);
        assert_eq!(default_settings.pixel_format(), AVPixelFormat::AV_PIX_FMT_NONE);
        assert!(default_settings.gop_size().is_none());
        assert!(default_settings.qmax().is_none());
        assert!(default_settings.qmin().is_none());
        assert!(default_settings.thread_count().is_none());
        assert!(default_settings.thread_type().is_none());
        assert!(default_settings.sample_aspect_ratio().is_none());
        assert!(default_settings.bitrate().is_none());
        assert!(default_settings.rc_min_rate().is_none());
        assert!(default_settings.rc_max_rate().is_none());
        assert!(default_settings.rc_buffer_size().is_none());
        assert!(default_settings.max_b_frames().is_none());
        assert!(default_settings.codec_specific_options().is_none());
        assert!(default_settings.flags().is_none());
        assert!(default_settings.flags2().is_none());
    }

    #[test]
    fn test_video_encoder_get_set_apply() {
        let width = 1920;
        let height = 1080;
        let frame_rate = 30;
        let pixel_format = AVPixelFormat::AV_PIX_FMT_YUV420P;
        let sample_aspect_ratio = AVRational { num: 1, den: 1 };
        let gop_size = 12;
        let qmax = 31;
        let qmin = 1;
        let thread_count = 4;
        let thread_type = 2;
        let bitrate = 8_000;
        let rc_min_rate = 500_000;
        let rc_max_rate = 2_000_000;
        let rc_buffer_size = 1024;
        let max_b_frames = 3;
        let mut codec_specific_options = Dictionary::new();
        let _ = codec_specific_options.set("preset", "ultrafast");
        let _ = codec_specific_options.set("crf", "23");
        let flags = 0x01;
        let flags2 = 0x02;

        let mut settings = VideoEncoderSettings::new();

        settings.set_width(width);
        settings.set_height(height);
        settings.set_frame_rate(frame_rate);
        settings.set_pixel_format(pixel_format);
        settings.set_sample_aspect_ratio(sample_aspect_ratio);
        settings.set_gop_size(gop_size);
        settings.set_qmax(qmax);
        settings.set_qmin(qmin);
        settings.set_thread_count(thread_count);
        settings.set_thread_type(thread_type);
        settings.set_bitrate(bitrate);
        settings.set_rc_min_rate(rc_min_rate);
        settings.set_rc_max_rate(rc_max_rate);
        settings.set_rc_buffer_size(rc_buffer_size);
        settings.set_max_b_frames(max_b_frames);
        settings.set_codec_specific_options(codec_specific_options);
        settings.set_flags(flags);
        settings.set_flags2(flags2);

        assert_eq!(settings.width(), width);
        assert_eq!(settings.height(), height);
        assert_eq!(settings.frame_rate(), frame_rate);
        assert_eq!(settings.pixel_format(), pixel_format);
        assert_eq!(settings.sample_aspect_ratio(), Some(sample_aspect_ratio));
        assert_eq!(settings.gop_size(), Some(gop_size));
        assert_eq!(settings.qmax(), Some(qmax));
        assert_eq!(settings.qmin(), Some(qmin));
        assert_eq!(settings.thread_count(), Some(thread_count));
        assert_eq!(settings.thread_type(), Some(thread_type));
        assert_eq!(settings.bitrate(), Some(bitrate));
        assert_eq!(settings.rc_min_rate(), Some(rc_min_rate));
        assert_eq!(settings.rc_max_rate(), Some(rc_max_rate));
        assert_eq!(settings.rc_buffer_size(), Some(rc_buffer_size));
        assert_eq!(settings.max_b_frames(), Some(max_b_frames));
        assert!(settings.codec_specific_options().is_some());
        let actual_codec_specific_options = settings.codec_specific_options().unwrap();
        assert_eq!(actual_codec_specific_options.get("preset").as_deref(), Some("ultrafast"));
        assert_eq!(actual_codec_specific_options.get("crf").as_deref(), Some("23"));
        assert_eq!(settings.flags(), Some(flags));
        assert_eq!(settings.flags2(), Some(flags2));

        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let result = settings.apply(&mut encoder);
        assert!(result.is_ok(), "Failed to apply settings: {:?}", result.err());

        assert_eq!(encoder.width, width);
        assert_eq!(encoder.height, height);
        assert_eq!(encoder.pix_fmt, pixel_format);
        assert_eq!(encoder.sample_aspect_ratio, sample_aspect_ratio);
        assert_eq!(encoder.framerate.num, frame_rate);
        assert_eq!(encoder.framerate.den, 1);
        assert_eq!(encoder.thread_count, thread_count);
        assert_eq!(encoder.thread_type, thread_type);
        assert_eq!(encoder.gop_size, gop_size);
        assert_eq!(encoder.qmax, qmax);
        assert_eq!(encoder.qmin, qmin);
        assert_eq!(encoder.bit_rate, bitrate);
        assert_eq!(encoder.rc_min_rate, rc_min_rate);
        assert_eq!(encoder.rc_max_rate, rc_max_rate);
        assert_eq!(encoder.rc_buffer_size, rc_buffer_size);
        assert_eq!(encoder.max_b_frames, max_b_frames);
        assert_eq!(encoder.flags, flags);
        assert_eq!(encoder.flags2, flags2);
    }

    #[test]
    fn test_video_encoder_settings_apply_error() {
        let settings = VideoEncoderSettings::default();
        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let result = settings.apply(&mut encoder);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            FfmpegError::Arguments("width, height, frame_rate and pixel_format must be set")
        );
    }

    #[test]
    fn test_video_encoder_average_duration() {
        let frame_rate = 30;
        let timebase = AVRational { num: 1, den: 30000 };
        let settings = VideoEncoderSettings {
            frame_rate,
            ..VideoEncoderSettings::default()
        };

        let expected_duration = (timebase.den as i64) / (frame_rate as i64 * timebase.num as i64);
        let actual_duration = settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_video_encoder_average_duration_with_custom_timebase() {
        let frame_rate = 60;
        let timebase = AVRational { num: 1, den: 60000 };

        let settings = VideoEncoderSettings {
            frame_rate,
            ..VideoEncoderSettings::default()
        };

        let expected_duration = (timebase.den as i64) / (frame_rate as i64 * timebase.num as i64);
        let actual_duration = settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_video_encoder_average_duration_with_zero_frame_rate() {
        let frame_rate = 0;
        let timebase = AVRational { num: 1, den: 30000 };
        let settings = VideoEncoderSettings {
            frame_rate,
            ..VideoEncoderSettings::default()
        };

        let actual_duration = settings.average_duration(timebase);
        assert_eq!(actual_duration, 0);
    }

    #[test]
    fn test_audio_encoder_settings_default() {
        let default_settings = AudioEncoderSettings::default();
        assert_eq!(default_settings.sample_rate(), 0);
        assert!(default_settings.ch_layout().is_none());
        assert_eq!(default_settings.sample_fmt(), AVSampleFormat::AV_SAMPLE_FMT_NONE);
        assert!(default_settings.thread_count().is_none());
        assert!(default_settings.thread_type().is_none());
        assert!(default_settings.bitrate().is_none());
        assert!(default_settings.buffer_size().is_none());
        assert!(default_settings.rc_min_rate().is_none());
        assert!(default_settings.rc_max_rate().is_none());
        assert!(default_settings.rc_buffer_size().is_none());
        assert!(default_settings.codec_specific_options().is_none());
        assert!(default_settings.flags().is_none());
        assert!(default_settings.flags2().is_none());
    }

    #[test]
    fn test_audio_encoder_get_set_apply() {
        let sample_rate = 44100;
        let channel_count = 2;
        let sample_fmt = AVSampleFormat::AV_SAMPLE_FMT_S16;
        let thread_count = 4;
        let thread_type = 1;
        let bitrate = 128_000;
        let buffer_size = 64 * 1024;
        let rc_min_rate = 64_000;
        let rc_max_rate = 256_000;
        let rc_buffer_size = 1024;
        let flags = 0x01;
        let flags2 = 0x02;

        let mut codec_specific_options = Dictionary::new();
        let _ = codec_specific_options.set("profile", "high");

        let mut settings = AudioEncoderSettings::new();
        settings.set_sample_rate(sample_rate);
        settings.set_channel_count(channel_count);
        settings.set_sample_fmt(sample_fmt);
        settings.set_thread_count(thread_count);
        settings.set_thread_type(thread_type);
        settings.set_bitrate(bitrate);
        settings.set_buffer_size(buffer_size);
        settings.set_rc_min_rate(rc_min_rate);
        settings.set_rc_max_rate(rc_max_rate);
        settings.set_rc_buffer_size(rc_buffer_size);
        settings.set_codec_specific_options(codec_specific_options);
        settings.set_flags(flags);
        settings.set_flags2(flags2);

        assert_eq!(settings.sample_rate(), sample_rate);
        assert_eq!(settings.channel_count(), 2);
        assert!(settings.ch_layout().is_some());
        assert_eq!(settings.sample_fmt(), sample_fmt);
        assert_eq!(settings.thread_count(), Some(thread_count));
        assert_eq!(settings.thread_type(), Some(thread_type));
        assert_eq!(settings.bitrate(), Some(bitrate));
        assert_eq!(settings.buffer_size(), Some(buffer_size));
        assert_eq!(settings.rc_min_rate(), Some(rc_min_rate));
        assert_eq!(settings.rc_max_rate(), Some(rc_max_rate));
        assert_eq!(settings.rc_buffer_size(), Some(rc_buffer_size));
        assert!(settings.codec_specific_options().is_some());

        let actual_codec_specific_options = settings.codec_specific_options().unwrap();
        assert_eq!(actual_codec_specific_options.get("profile").as_deref(), Some("high"));

        assert_eq!(settings.flags(), Some(flags));
        assert_eq!(settings.flags2(), Some(flags2));

        let custom_layout = AVChannelLayout {
            order: AVChannelOrder::AV_CHANNEL_ORDER_NATIVE,
            nb_channels: 2,
            u: ffmpeg_sys_next::AVChannelLayout__bindgen_ty_1 {
                mask: AV_CH_LAYOUT_STEREO,
            },
            opaque: std::ptr::null_mut(),
        };

        assert!(
            settings.set_ch_layout(custom_layout).is_ok(),
            "Failed to set custom channel layout"
        );
        let layout = settings.ch_layout().expect("Expected ch_layout to be set.");
        assert_eq!(layout.nb_channels, 2, "Expected channel layout to have 2 channels.");
        assert_eq!(
            unsafe { layout.u.mask },
            AV_CH_LAYOUT_STEREO,
            "Expected channel mask to match AV_CH_LAYOUT_STEREO."
        );
        assert_eq!(
            layout.order,
            AVChannelOrder::AV_CHANNEL_ORDER_NATIVE,
            "Expected channel order to be AV_CHANNEL_ORDER_NATIVE."
        );
        assert_eq!(settings.channel_count(), 2, "Expected channel_count to return 2.");

        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let result = settings.apply(&mut encoder);

        assert!(result.is_ok(), "Failed to apply settings: {:?}", result.err());
        assert_eq!(encoder.sample_rate, sample_rate);
        assert_eq!(encoder.ch_layout.nb_channels, channel_count);
        assert_eq!(encoder.sample_fmt, sample_fmt);
        assert_eq!(encoder.thread_count, thread_count);
        assert_eq!(encoder.thread_type, thread_type);
        assert_eq!(encoder.bit_rate, bitrate);
        assert_eq!(encoder.rc_min_rate, rc_min_rate);
        assert_eq!(encoder.rc_max_rate, rc_max_rate);
        assert_eq!(encoder.rc_buffer_size, rc_buffer_size);
        assert_eq!(encoder.flags, flags);
        assert_eq!(encoder.flags2, flags2);
    }

    #[test]
    fn test_set_ch_layout_invalid_layout() {
        let mut settings = AudioEncoderSettings::new();
        let invalid_layout = ffmpeg_sys_next::AVChannelLayout {
            order: ffmpeg_sys_next::AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC,
            nb_channels: 0,
            u: ffmpeg_sys_next::AVChannelLayout__bindgen_ty_1 { mask: 0 },
            opaque: std::ptr::null_mut(),
        };

        assert!(
            settings.set_ch_layout(invalid_layout).is_err(),
            "Expected an error for invalid channel layout."
        );
    }

    #[test]
    fn test_audio_encoder_settings_apply_error() {
        let settings = AudioEncoderSettings::default();
        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let result = settings.apply(&mut encoder);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            FfmpegError::Arguments("sample_rate, channel_layout and sample_fmt must be set")
        );
    }

    #[test]
    fn test_average_duration() {
        let sample_rate = 48000;
        let timebase = AVRational { num: 1, den: 48000 };
        let settings = AudioEncoderSettings {
            sample_rate,
            ..AudioEncoderSettings::default()
        };
        let expected_duration = 1;
        let actual_duration = settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_average_duration_with_zero_sample_rate() {
        let sample_rate = 0;
        let timebase = AVRational { num: 1, den: 48000 };
        let settings = AudioEncoderSettings {
            sample_rate,
            ..AudioEncoderSettings::default()
        };
        let actual_duration = settings.average_duration(timebase);

        assert_eq!(actual_duration, 0);
    }

    #[test]
    fn test_average_duration_with_custom_timebase() {
        let sample_rate = 96000;
        let timebase = AVRational { num: 1, den: 96000 };
        let settings = AudioEncoderSettings {
            sample_rate,
            ..AudioEncoderSettings::default()
        };
        let expected_duration = 1;
        let actual_duration = settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_encoder_settings_apply_video() {
        let sample_aspect_ratio = AVRational { num: 1, den: 1 };
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            sample_aspect_ratio: Some(sample_aspect_ratio),
            gop_size: Some(12),
            ..VideoEncoderSettings::default()
        };
        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let encoder_settings = EncoderSettings::Video(video_settings);
        let result = encoder_settings.apply(&mut encoder);

        assert!(result.is_ok(), "Failed to apply video settings: {:?}", result.err());
        assert_eq!(encoder.width, 1920);
        assert_eq!(encoder.height, 1080);
        assert_eq!(encoder.pix_fmt, AVPixelFormat::AV_PIX_FMT_YUV420P);
        assert_eq!(encoder.sample_aspect_ratio.num, 1);
        assert_eq!(encoder.sample_aspect_ratio.den, 1);
    }

    #[test]
    fn test_encoder_settings_apply_audio() {
        let audio_settings = AudioEncoderSettings {
            sample_rate: 44100,
            ch_layout: Some(SmartObject::new(unsafe { std::mem::zeroed() }, |ptr| unsafe {
                av_channel_layout_uninit(ptr)
            })),
            sample_fmt: AVSampleFormat::AV_SAMPLE_FMT_FLTP,
            thread_count: Some(4),
            ..AudioEncoderSettings::default()
        };
        let mut encoder = unsafe { std::mem::zeroed::<AVCodecContext>() };
        let encoder_settings = EncoderSettings::Audio(audio_settings);
        let result = encoder_settings.apply(&mut encoder);

        assert!(result.is_ok(), "Failed to apply audio settings: {:?}", result.err());
        assert_eq!(encoder.sample_rate, 44100);
        assert_eq!(encoder.sample_fmt, AVSampleFormat::AV_SAMPLE_FMT_FLTP);
        assert_eq!(encoder.thread_count, 4);
    }

    #[test]
    fn test_encoder_settings_codec_specific_options() {
        let mut video_codec_options = Dictionary::new();
        let _ = video_codec_options.set("preset", "fast");

        let video_settings = VideoEncoderSettings {
            codec_specific_options: Some(video_codec_options.clone()),
            ..VideoEncoderSettings::default()
        };
        let mut encoder_settings = EncoderSettings::Video(video_settings);
        let options = encoder_settings.codec_specific_options();

        assert!(options.is_some());
        assert_eq!(options.unwrap().get("preset").as_deref(), Some("fast"));

        let mut audio_codec_options = Dictionary::new();
        let _ = audio_codec_options.set("bitrate", "128k");
        let audio_settings = AudioEncoderSettings {
            codec_specific_options: Some(audio_codec_options.clone()),
            ..AudioEncoderSettings::default()
        };
        let mut encoder_settings = EncoderSettings::Audio(audio_settings);
        let options = encoder_settings.codec_specific_options();

        assert!(options.is_some());
        assert_eq!(options.unwrap().get("bitrate").as_deref(), Some("128k"));
    }

    #[test]
    fn test_encoder_settings_average_duration_video() {
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            ..VideoEncoderSettings::default()
        };
        let encoder_settings = EncoderSettings::Video(video_settings);
        let timebase = AVRational { num: 1, den: 30000 };
        let expected_duration = (timebase.den as i64) / (30 * timebase.num as i64);
        let actual_duration = encoder_settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_encoder_settings_average_duration_audio() {
        let audio_settings = AudioEncoderSettings {
            sample_rate: 44100,
            ..AudioEncoderSettings::default()
        };
        let encoder_settings = EncoderSettings::Audio(audio_settings);
        let timebase = AVRational { num: 1, den: 44100 };
        let expected_duration = (timebase.den as i64) / (44100 * timebase.num as i64);
        let actual_duration = encoder_settings.average_duration(timebase);

        assert_eq!(actual_duration, expected_duration);
    }

    #[test]
    fn test_from_video_encoder_settings() {
        let sample_aspect_ratio = AVRational { num: 1, den: 1 };
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            sample_aspect_ratio: Some(sample_aspect_ratio),
            gop_size: Some(12),
            ..VideoEncoderSettings::default()
        };
        let encoder_settings: EncoderSettings = video_settings.into();

        if let EncoderSettings::Video(actual_video_settings) = encoder_settings {
            assert_eq!(actual_video_settings.width(), 1920);
            assert_eq!(actual_video_settings.height(), 1080);
            assert_eq!(actual_video_settings.frame_rate(), 30);
            assert_eq!(actual_video_settings.pixel_format(), AVPixelFormat::AV_PIX_FMT_YUV420P);
            assert_eq!(actual_video_settings.sample_aspect_ratio(), Some(sample_aspect_ratio));
            assert_eq!(actual_video_settings.gop_size(), Some(12));
        } else {
            panic!("Expected EncoderSettings::Video variant");
        }
    }

    #[test]
    fn test_from_audio_encoder_settings() {
        let audio_settings = AudioEncoderSettings {
            sample_rate: 44100,
            ch_layout: Some(SmartObject::new(unsafe { std::mem::zeroed() }, |ptr| unsafe {
                av_channel_layout_uninit(ptr)
            })),
            sample_fmt: AVSampleFormat::AV_SAMPLE_FMT_FLTP,
            thread_count: Some(4),
            ..AudioEncoderSettings::default()
        };
        let encoder_settings: EncoderSettings = audio_settings.into();

        if let EncoderSettings::Audio(actual_audio_settings) = encoder_settings {
            assert_eq!(actual_audio_settings.sample_rate(), 44100);
            assert!(actual_audio_settings.ch_layout.is_some());
            assert_eq!(actual_audio_settings.sample_fmt(), AVSampleFormat::AV_SAMPLE_FMT_FLTP);
            assert_eq!(actual_audio_settings.thread_count(), Some(4));
        } else {
            panic!("Expected EncoderSettings::Audio variant");
        }
    }

    #[test]
    fn test_encoder_new_with_null_codec() {
        let codec = EncoderCodec::from_ptr(std::ptr::null());
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let settings = VideoEncoderSettings::default();
        let result = Encoder::new(codec, &mut output, incoming_time_base, outgoing_time_base, settings);

        assert!(matches!(result, Err(FfmpegError::NoEncoder)));
    }

    #[test]
    fn test_encoder_new_success() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4);
        assert!(codec.is_some(), "Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..VideoEncoderSettings::default()
        };
        let result = Encoder::new(codec.unwrap(), &mut output, incoming_time_base, outgoing_time_base, settings);

        assert!(result.is_ok(), "Encoder creation failed: {:?}", result.err());

        let encoder = result.unwrap();
        assert_eq!(encoder.incoming_time_base.num, 1);
        assert_eq!(encoder.incoming_time_base.den, 1000);
        assert_eq!(encoder.outgoing_time_base.num, 1);
        assert_eq!(encoder.outgoing_time_base.den, 1000);
        assert_eq!(encoder.stream_index, 0);
    }

    #[test]
    fn test_send_eof() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4).expect("Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let video_settings = VideoEncoderSettings {
            width: 640,
            height: 480,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..Default::default()
        };
        let mut encoder = Encoder::new(
            codec,
            &mut output,
            AVRational { num: 1, den: 1000 },
            AVRational { num: 1, den: 1000 },
            video_settings,
        )
        .expect("Failed to create encoder");

        let result = encoder.send_eof();
        assert!(result.is_ok(), "send_eof returned an error: {:?}", result.err());
        assert!(encoder.send_eof().is_err(), "send_eof should return an error");
    }

    #[test]
    fn test_encoder_getters() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4).expect("Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let video_settings = VideoEncoderSettings {
            width: 640,
            height: 480,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..Default::default()
        };
        let encoder = Encoder::new(codec, &mut output, incoming_time_base, outgoing_time_base, video_settings)
            .expect("Failed to create encoder");

        let stream_index = encoder.stream_index();
        assert_eq!(stream_index, 0, "Unexpected stream index: expected 0, got {}", stream_index);

        let actual_incoming_time_base = encoder.incoming_time_base();
        assert_eq!(
            actual_incoming_time_base, incoming_time_base,
            "Unexpected incoming_time_base: expected {:?}, got {:?}",
            incoming_time_base, actual_incoming_time_base
        );

        let actual_outgoing_time_base = encoder.outgoing_time_base();
        assert_eq!(
            actual_outgoing_time_base, outgoing_time_base,
            "Unexpected outgoing_time_base: expected {:?}, got {:?}",
            outgoing_time_base, actual_outgoing_time_base
        );
    }

    #[test]
    fn test_muxer_settings_default() {
        let settings = MuxerSettings::default();

        assert!(settings.interleave, "Default interleave should be true");
        assert!(
            settings.muxer_options.is_empty(),
            "Default muxer_options should be an empty dictionary"
        );
    }

    #[test]
    fn test_muxer_settings_builder_custom_values() {
        let mut custom_options = Dictionary::new();
        custom_options.set("key1", "value1").unwrap();
        custom_options.set("key2", "value2").unwrap();
        let settings = MuxerSettings::builder()
            .interleave(false)
            .muxer_options(custom_options.clone())
            .build();

        assert!(!settings.interleave, "Interleave should be set to false");
        assert_eq!(
            settings.muxer_options.get("key1").as_deref(),
            Some("value1"),
            "Expected muxer_options to have key1 with value 'value1'"
        );
        assert_eq!(
            settings.muxer_options.get("key2").as_deref(),
            Some("value2"),
            "Expected muxer_options to have key2 with value 'value2'"
        );
    }

    #[test]
    fn test_muxer_encoder_new() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4).expect("Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data, options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..Default::default()
        };
        let encoder_settings: EncoderSettings = video_settings.into();
        let mut muxer_options = Dictionary::new();
        muxer_options.set("option1", "value1").unwrap();
        muxer_options.set("option2", "value2").unwrap();
        let muxer_settings = MuxerSettings {
            interleave: false,
            muxer_options,
        };
        let muxer_encoder = MuxerEncoder::new(
            codec,
            output,
            incoming_time_base,
            outgoing_time_base,
            encoder_settings,
            muxer_settings,
        );

        assert!(
            muxer_encoder.is_ok(),
            "Failed to create MuxerEncoder: {:?}",
            muxer_encoder.err()
        );
        let muxer_encoder = muxer_encoder.unwrap();

        assert!(!muxer_encoder.interleave, "Expected interleave to be false, but it was true");
        assert_eq!(
            muxer_encoder.muxer_options.get("option1").as_deref(),
            Some("value1"),
            "Expected muxer_options to have 'option1' with value 'value1'"
        );
        assert_eq!(
            muxer_encoder.muxer_options.get("option2").as_deref(),
            Some("value2"),
            "Expected muxer_options to have 'option2' with value 'value2'"
        );
        assert!(
            !muxer_encoder.muxer_headers_written,
            "Expected muxer_headers_written to be false"
        );
        assert_eq!(muxer_encoder.previous_dts, -1, "Expected previous_dts to be -1");
        assert_eq!(muxer_encoder.previous_pts, -1, "Expected previous_pts to be -1");
        assert!(muxer_encoder.buffered_packet.is_none(), "Expected buffered_packet to be None");
        assert_eq!(muxer_encoder.stream_index(), 0, "Expected stream index to be 0");
        assert_eq!(
            muxer_encoder.incoming_time_base(),
            incoming_time_base,
            "Expected incoming_time_base to match"
        );
        assert_eq!(
            muxer_encoder.outgoing_time_base(),
            outgoing_time_base,
            "Expected outgoing_time_base to match"
        );
    }

    #[test]
    fn test_muxer_encoder_into_inner() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4).expect("Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data.clone(), options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..Default::default()
        };
        let encoder_settings: EncoderSettings = video_settings.into();
        let muxer_settings = MuxerSettings::default();
        let muxer_encoder = MuxerEncoder::new(
            codec,
            output,
            incoming_time_base,
            outgoing_time_base,
            encoder_settings,
            muxer_settings,
        )
        .expect("Failed to create MuxerEncoder");

        let output = muxer_encoder.into_inner();
        assert_eq!(
            output.into_inner(),
            data,
            "Expected the inner output data to match the original data"
        );
    }

    #[test]
    fn test_muxer_encoder_deref() {
        let codec = EncoderCodec::new(AV_CODEC_ID_MPEG4).expect("Failed to find MPEG-4 encoder");
        let data = std::io::Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data, options).expect("Failed to create Output");
        let incoming_time_base = AVRational { num: 1, den: 1000 };
        let outgoing_time_base = AVRational { num: 1, den: 1000 };
        let video_settings = VideoEncoderSettings {
            width: 1920,
            height: 1080,
            frame_rate: 30,
            pixel_format: AVPixelFormat::AV_PIX_FMT_YUV420P,
            ..Default::default()
        };
        let encoder_settings: EncoderSettings = video_settings.into();
        let muxer_settings = MuxerSettings::default();
        let mut muxer_encoder = MuxerEncoder::new(
            codec,
            output,
            incoming_time_base,
            outgoing_time_base,
            encoder_settings,
            muxer_settings,
        )
        .expect("Failed to create MuxerEncoder");

        let encoder_ref = &*muxer_encoder;
        assert_eq!(
            encoder_ref.stream_index(),
            0,
            "Expected stream index to be 0, but got {}",
            encoder_ref.stream_index()
        );

        let encoder_mut_ref = &mut *muxer_encoder;
        encoder_mut_ref.previous_dts = 12345;
        assert_eq!(
            encoder_mut_ref.previous_dts, 12345,
            "Expected previous_dts to be updated to 12345, but got {}",
            encoder_mut_ref.previous_dts
        );
    }
}
