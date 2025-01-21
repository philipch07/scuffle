use ffmpeg_sys_next::*;

use crate::error::FfmpegError;
use crate::smart_object::SmartPtr;
use crate::utils::check_i64;

#[derive(Debug)]
pub struct Packets<'a> {
    context: &'a mut AVFormatContext,
}

/// Safety: `Packets` is safe to send between threads.
unsafe impl Send for Packets<'_> {}

impl<'a> Packets<'a> {
    pub fn new(context: &'a mut AVFormatContext) -> Self {
        Self { context }
    }

    pub fn receive(&mut self) -> Result<Option<Packet>, FfmpegError> {
        let mut packet = Packet::new()?;

        // Safety: av_read_frame is safe to call, 'packet' is a valid pointer
        let ret = unsafe { av_read_frame(self.context, packet.as_mut_ptr()) };

        match ret {
            0 => Ok(Some(packet)),
            AVERROR_EOF => Ok(None),
            _ => Err(FfmpegError::Code(ret.into())),
        }
    }
}

impl Iterator for Packets<'_> {
    type Item = Result<Packet, FfmpegError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive().transpose()
    }
}

pub struct Packet(SmartPtr<AVPacket>);

/// Safety: `Packet` is safe to send between threads.
unsafe impl Send for Packet {}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Packet")
            .field("stream_index", &self.stream_index())
            .field("pts", &self.pts())
            .field("dts", &self.dts())
            .field("duration", &self.duration())
            .field("pos", &self.pos())
            .field("is_key", &self.is_key())
            .field("is_corrupt", &self.is_corrupt())
            .field("is_discard", &self.is_discard())
            .field("is_trusted", &self.is_trusted())
            .field("is_disposable", &self.is_disposable())
            .finish()
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        unsafe { Self::wrap(av_packet_clone(self.0.as_ptr())).expect("failed to clone packet") }
    }
}

impl Packet {
    pub fn new() -> Result<Self, FfmpegError> {
        // Safety: av_packet_alloc is safe to call, and the pointer it returns is valid.
        unsafe { Self::wrap(av_packet_alloc()) }.ok_or(FfmpegError::Alloc)
    }

    /// Safety: `ptr` must be a valid pointer to a packet.
    unsafe fn wrap(ptr: *mut AVPacket) -> Option<Self> {
        Some(Self(SmartPtr::wrap_non_null(ptr, |ptr| av_packet_free(ptr))?))
    }

    pub fn as_ptr(&self) -> *const AVPacket {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVPacket {
        self.0.as_mut_ptr()
    }

    pub fn stream_index(&self) -> i32 {
        self.0.as_deref_except().stream_index
    }

    pub fn set_stream_index(&mut self, stream_index: i32) {
        self.0.as_deref_mut_except().stream_index = stream_index as _;
    }

    pub fn pts(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().pts)
    }

    pub fn set_pts(&mut self, pts: Option<i64>) {
        self.0.as_deref_mut_except().pts = pts.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn dts(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().dts)
    }

    pub fn set_dts(&mut self, dts: Option<i64>) {
        self.0.as_deref_mut_except().dts = dts.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn duration(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().duration)
    }

    pub fn set_duration(&mut self, duration: Option<i64>) {
        self.0.as_deref_mut_except().duration = duration.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn rescale_timebase(&mut self, from: AVRational, to: AVRational) {
        // Safety: av_rescale_q_rnd is safe to call
        self.set_pts(
            self.pts()
                .map(|pts| unsafe { av_rescale_q_rnd(pts, from, to, AVRounding::AV_ROUND_NEAR_INF) }),
        );

        // Safety: av_rescale_q_rnd is safe to call
        self.set_dts(
            self.dts()
                .map(|dts| unsafe { av_rescale_q_rnd(dts, from, to, AVRounding::AV_ROUND_NEAR_INF) }),
        );

        // Safety: av_rescale_q is safe to call
        self.set_duration(self.duration().map(|duration| unsafe { av_rescale_q(duration, from, to) }));
    }

    pub fn pos(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().pos)
    }

    pub fn set_pos(&mut self, pos: Option<i64>) {
        self.0.as_deref_mut_except().pos = pos.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn data(&self) -> &[u8] {
        if self.0.as_deref_except().size <= 0 {
            return &[];
        }

        // Safety: `self.0` is a valid pointer.
        unsafe { std::slice::from_raw_parts(self.0.as_deref_except().data, self.0.as_deref_except().size as usize) }
    }

    pub fn is_key(&self) -> bool {
        self.0.as_deref_except().flags & AV_PKT_FLAG_KEY != 0
    }

    pub fn is_corrupt(&self) -> bool {
        self.0.as_deref_except().flags & AV_PKT_FLAG_CORRUPT != 0
    }

    pub fn is_discard(&self) -> bool {
        self.0.as_deref_except().flags & AV_PKT_FLAG_DISCARD != 0
    }

    pub fn is_trusted(&self) -> bool {
        self.0.as_deref_except().flags & AV_PKT_FLAG_TRUSTED != 0
    }

    pub fn is_disposable(&self) -> bool {
        self.0.as_deref_except().flags & AV_PKT_FLAG_DISPOSABLE != 0
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use ffmpeg_sys_next::AVRational;
    use insta::assert_debug_snapshot;

    use crate::packet::Packet;

    #[test]
    fn test_packet_clone_snapshot() {
        let mut original_packet = Packet::new().expect("Failed to create original Packet");
        original_packet.set_stream_index(1);
        original_packet.set_pts(Some(12345));
        original_packet.set_dts(Some(54321));
        original_packet.set_duration(Some(1000));
        original_packet.set_pos(Some(2000));

        let cloned_packet = original_packet.clone();

        assert_debug_snapshot!(cloned_packet, @r"
        Packet {
            stream_index: 1,
            pts: Some(
                12345,
            ),
            dts: Some(
                54321,
            ),
            duration: Some(
                1000,
            ),
            pos: Some(
                2000,
            ),
            is_key: false,
            is_corrupt: false,
            is_discard: false,
            is_trusted: false,
            is_disposable: false,
        }
        ");

        // ensure cloned packet is independent
        original_packet.set_pts(Some(99999));
        assert_ne!(
            cloned_packet.pts(),
            original_packet.pts(),
            "Expected cloned packet PTS to remain unchanged after modifying the original"
        );

        assert_debug_snapshot!(original_packet, @r"
        Packet {
            stream_index: 1,
            pts: Some(
                99999,
            ),
            dts: Some(
                54321,
            ),
            duration: Some(
                1000,
            ),
            pos: Some(
                2000,
            ),
            is_key: false,
            is_corrupt: false,
            is_discard: false,
            is_trusted: false,
            is_disposable: false,
        }
        ");
    }

    #[test]
    fn test_packet_as_ptr() {
        let packet = Packet::new().expect("Failed to create Packet");
        let raw_ptr = packet.as_ptr();

        assert!(!raw_ptr.is_null(), "Expected a non-null pointer from Packet::as_ptr");
        unsafe {
            assert_eq!(
                (*raw_ptr).stream_index,
                0,
                "Expected the default stream_index to be 0 for a new Packet"
            );
        }
    }

    #[test]
    fn test_packet_rescale_timebase() {
        let mut packet = Packet::new().expect("Failed to create Packet");
        packet.set_pts(Some(1000));
        packet.set_dts(Some(900));
        packet.set_duration(Some(100));
        let from_time_base = AVRational { num: 1, den: 1000 };
        let to_time_base = AVRational { num: 1, den: 48000 };

        packet.rescale_timebase(from_time_base, to_time_base);
        assert_debug_snapshot!(packet, @r"
        Packet {
            stream_index: 0,
            pts: Some(
                48000,
            ),
            dts: Some(
                43200,
            ),
            duration: Some(
                4800,
            ),
            pos: Some(
                -1,
            ),
            is_key: false,
            is_corrupt: false,
            is_discard: false,
            is_trusted: false,
            is_disposable: false,
        }
        ");
    }

    #[test]
    fn test_packet_data_empty() {
        let mut packet = Packet::new().expect("Failed to create Packet");
        unsafe {
            let av_packet = packet.as_mut_ptr().as_mut().unwrap();
            av_packet.size = 0;
        }

        let data = packet.data();

        assert!(
            data.is_empty(),
            "Expected the data slice to be empty when packet size is zero"
        );
    }
}
