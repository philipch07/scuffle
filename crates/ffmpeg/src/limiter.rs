use ffmpeg_sys_next::AVRational;

use crate::frame::Frame;

#[derive(Debug)]
pub struct FrameRateLimiter {
    last_frame: i64,
    accumulated_time: i64,
    frame_timing: i64,
}

impl FrameRateLimiter {
    /// Creates a new frame rate limiter.
    pub const fn new(frame_rate: i32, time_base: AVRational) -> Self {
        let frame_timing = ((time_base.den / frame_rate) / time_base.num) as i64;
        Self {
            last_frame: 0,
            accumulated_time: 0,
            frame_timing,
        }
    }

    /// Limits the frame rate.
    pub const fn limit(&mut self, frame: &Frame) -> bool {
        let ts = match frame.dts() {
            Some(dts) => dts,
            None => frame.pts().unwrap(),
        };

        let delta = ts - self.last_frame;
        self.last_frame = ts;
        self.accumulated_time += delta;
        if self.accumulated_time >= self.frame_timing {
            self.accumulated_time -= self.frame_timing;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use ffmpeg_sys_next::AVRational;

    use crate::frame::Frame;
    use crate::limiter::FrameRateLimiter;

    #[test]
    fn test_frame_rate_limiter_new() {
        let frame_rate = 30;
        let time_base = AVRational { num: 1, den: 30000 };

        let limiter = FrameRateLimiter::new(frame_rate, time_base);

        assert_eq!(limiter.last_frame, 0);
        assert_eq!(limiter.accumulated_time, 0);
        assert_eq!(limiter.frame_timing, 1000);
    }

    #[test]
    fn test_frame_rate_limiter_limit_if_case() {
        let frame_rate = 30;
        let time_base = AVRational { num: 1, den: 30000 };
        let mut limiter = FrameRateLimiter::new(frame_rate, time_base);
        let mut frame = Frame::new().unwrap();
        frame.set_dts(Some(2000));
        let result = limiter.limit(&frame);

        assert!(result);
        assert_eq!(limiter.last_frame, 2000);
        assert_eq!(limiter.accumulated_time, 1000);
    }

    #[test]
    fn test_frame_rate_limiter_limit_else_case() {
        let frame_rate = 30;
        let time_base = AVRational { num: 1, den: 30000 };
        let mut limiter = FrameRateLimiter::new(frame_rate, time_base);
        let mut frame = Frame::new().unwrap();
        frame.set_dts(Some(500));
        let result = limiter.limit(&frame);

        assert!(!result);
        assert_eq!(limiter.last_frame, 500);
        assert_eq!(limiter.accumulated_time, 500);
    }
}
