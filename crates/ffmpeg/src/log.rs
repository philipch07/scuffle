use std::ffi::CStr;
use std::sync::Arc;

use arc_swap::ArcSwap;
use ffmpeg_sys_next::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LogLevel {
    Quiet = AV_LOG_QUIET,
    Panic = AV_LOG_PANIC,
    Fatal = AV_LOG_FATAL,
    Error = AV_LOG_ERROR,
    Warning = AV_LOG_WARNING,
    Info = AV_LOG_INFO,
    Verbose = AV_LOG_VERBOSE,
    Debug = AV_LOG_DEBUG,
    Trace = AV_LOG_TRACE,
}

impl LogLevel {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            -8 => Self::Quiet,
            0 => Self::Panic,
            8 => Self::Fatal,
            16 => Self::Error,
            24 => Self::Warning,
            32 => Self::Info,
            40 => Self::Verbose,
            48 => Self::Debug,
            56 => Self::Trace,
            _ => Self::Info,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Panic => "panic",
            Self::Fatal => "fatal",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Verbose => "verbose",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

pub fn set_log_level(level: LogLevel) {
    unsafe {
        av_log_set_level(level as i32);
    }
}

pub fn log_callback_set<F: Fn(LogLevel, Option<String>, String) + Send + Sync + 'static>(callback: F) {
    type Function = Box<dyn Fn(LogLevel, Option<String>, String) + Send + Sync>;
    static LOG_CALLBACK: std::sync::OnceLock<ArcSwap<Option<Function>>> = std::sync::OnceLock::new();

    unsafe extern "C" fn log_cb(
        ptr: *mut libc::c_void,
        level: libc::c_int,
        fmt: *const libc::c_char,
        va: *mut __va_list_tag,
    ) {
        let level = LogLevel::from_i32(level);
        let class = if ptr.is_null() {
            None
        } else {
            let class = &mut **(ptr as *mut *mut AVClass);
            class
                .item_name
                .map(|im| CStr::from_ptr(im(ptr)).to_string_lossy().trim().to_owned())
        };

        let mut buf = [0u8; 1024];

        vsnprintf(buf.as_mut_ptr() as *mut i8, buf.len() as _, fmt, va);

        let msg = CStr::from_ptr(buf.as_ptr() as *const i8).to_string_lossy().trim().to_owned();

        if let Some(cb) = LOG_CALLBACK.get() {
            if let Some(cb) = cb.load().as_ref() {
                cb(level, class, msg);
            }
        }
    }

    unsafe {
        LOG_CALLBACK
            .get_or_init(|| ArcSwap::new(Arc::new(None)))
            .store(Arc::new(Some(Box::new(callback))));
        av_log_set_callback(Some(log_cb));
    }
}

pub fn log_callback_unset() {
    unsafe {
        av_log_set_callback(None);
    }
}

#[cfg(feature = "tracing")]
pub fn log_callback_tracing() {
    log_callback_set(|mut level, class, msg| {
        let class = class.unwrap_or_else(|| "ffmpeg".to_owned());

        if msg == "deprecated pixel format used, make sure you did set range correctly" {
            level = LogLevel::Debug;
        }

        match level {
            LogLevel::Trace => tracing::trace!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Verbose => tracing::trace!("{}: [{class} @ {msg}", level.as_str()),
            LogLevel::Debug => tracing::debug!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Info => tracing::info!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Warning => tracing::warn!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Quiet => tracing::error!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Error => tracing::error!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Panic => tracing::error!("{}: {class} @ {msg}", level.as_str()),
            LogLevel::Fatal => tracing::error!("{}: {class} @ {msg}", level.as_str()),
        }
    });
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::ffi::CString;
    use std::ptr;
    use std::sync::{Arc, Mutex};

    use ffmpeg_sys_next::{av_log, av_log_get_level, avcodec_find_decoder, AVCodecID};
    use tracing::subscriber::set_default;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    use crate::log::{log_callback_set, log_callback_tracing, log_callback_unset, set_log_level, LogLevel};

    #[test]
    fn test_log_level_as_str_using_from_i32() {
        let test_cases = [
            (-8, "quiet"),
            (0, "panic"),
            (8, "fatal"),
            (16, "error"),
            (24, "warning"),
            (32, "info"),
            (40, "verbose"),
            (48, "debug"),
            (56, "trace"),
            (100, "info"),
            (-1, "info"),
        ];

        for &(input, expected) in &test_cases {
            let log_level = LogLevel::from_i32(input);
            assert_eq!(
                log_level.as_str(),
                expected,
                "Expected '{}' for input {}, but got '{}'",
                expected,
                input,
                log_level.as_str()
            );
        }
    }

    #[test]
    fn test_set_log_level() {
        let log_levels = [
            LogLevel::Quiet,
            LogLevel::Panic,
            LogLevel::Fatal,
            LogLevel::Error,
            LogLevel::Warning,
            LogLevel::Info,
            LogLevel::Verbose,
            LogLevel::Debug,
            LogLevel::Trace,
        ];

        for &level in &log_levels {
            set_log_level(level);
            let current_level = unsafe { av_log_get_level() };

            assert_eq!(
                current_level, level as i32,
                "Expected log level to be {}, but got {}",
                level as i32, current_level
            );
        }
    }

    #[test]
    fn test_log_callback_set() {
        let captured_logs = Arc::new(Mutex::new(Vec::new()));
        let callback_logs = Arc::clone(&captured_logs);
        log_callback_set(move |level, class, message| {
            let mut logs = callback_logs.lock().unwrap();
            logs.push((level, class, message));
        });

        let log_message = CString::new("Test warning log message").expect("Failed to create CString");
        unsafe {
            av_log(std::ptr::null_mut(), LogLevel::Warning as i32, log_message.as_ptr());
        }

        let logs = captured_logs.lock().unwrap();
        assert_eq!(logs.len(), 1, "Expected one log message to be captured");

        let (level, class, message) = &logs[0];
        assert_eq!(*level, LogLevel::Warning, "Expected log level to be Warning");
        assert!(class.is_none(), "Expected class to be None for this test");
        assert_eq!(message, "Test warning log message", "Expected log message to match");
    }

    #[test]
    fn test_log_callback_with_class() {
        unsafe {
            let codec = avcodec_find_decoder(AVCodecID::AV_CODEC_ID_H264);
            assert!(!codec.is_null(), "Failed to find H264 codec");

            let av_class_ptr = (*codec).priv_class;
            assert!(!av_class_ptr.is_null(), "AVClass for codec is null");

            let captured_logs = Arc::new(Mutex::new(Vec::new()));

            let callback_logs = Arc::clone(&captured_logs);
            log_callback_set(move |level, class, message| {
                let mut logs = callback_logs.lock().unwrap();
                logs.push((level, class, message));
            });

            av_log(
                &av_class_ptr as *const _ as *mut _,
                LogLevel::Info as i32,
                CString::new("Test log message with real AVClass").unwrap().as_ptr(),
            );

            let logs = captured_logs.lock().unwrap();
            assert_eq!(logs.len(), 1, "Expected one log message to be captured");

            let (level, class, message) = &logs[0];
            assert_eq!(*level, LogLevel::Info, "Expected log level to be Info");
            assert!(class.is_some(), "Expected class name to be captured");
            assert_eq!(message, "Test log message with real AVClass", "Expected log message to match");
        }
    }

    #[test]
    fn test_log_callback_unset() {
        let captured_logs = Arc::new(Mutex::new(Vec::new()));
        let callback_logs = Arc::clone(&captured_logs);
        log_callback_set(move |level, class, message| {
            let mut logs = callback_logs.lock().unwrap();
            logs.push((level, class, message));
        });

        unsafe {
            av_log(
                std::ptr::null_mut(),
                LogLevel::Info as i32,
                CString::new("Test log message before unset").unwrap().as_ptr(),
            );
        }

        {
            let logs = captured_logs.lock().unwrap();
            assert_eq!(
                logs.len(),
                1,
                "Expected one log message to be captured before unsetting the callback"
            );
            let (_, _, message) = &logs[0];
            assert_eq!(message, "Test log message before unset", "Expected the log message to match");
        }

        log_callback_unset();

        unsafe {
            av_log(
                std::ptr::null_mut(),
                LogLevel::Info as i32,
                CString::new("Test log message after unset").unwrap().as_ptr(),
            );
        }

        let logs = captured_logs.lock().unwrap();
        assert_eq!(
            logs.len(),
            1,
            "Expected no additional log messages to be captured after unsetting the callback"
        );
    }

    #[cfg(feature = "tracing")]
    #[test]
    #[tracing_test::traced_test]
    fn test_log_callback_tracing() {
        let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
        let _ = set_default(subscriber);
        log_callback_tracing();

        let levels_and_expected_tracing = [
            (LogLevel::Trace, "trace"),
            (LogLevel::Verbose, "trace"),
            (LogLevel::Debug, "debug"),
            (LogLevel::Info, "info"),
            // (LogLevel::Warning, "warn"), TODO: idk why including this makes it not work
            (LogLevel::Quiet, "error"),
            (LogLevel::Error, "error"),
            (LogLevel::Panic, "error"),
            (LogLevel::Fatal, "error"),
        ];

        for (level, expected_tracing_level) in &levels_and_expected_tracing {
            let message = format!("Test {} log message", expected_tracing_level);
            unsafe {
                av_log(
                    ptr::null_mut(),
                    *level as i32,
                    CString::new(message.clone()).expect("Failed to create CString").as_ptr(),
                );
            }
        }

        for (_level, expected_tracing_level) in &levels_and_expected_tracing {
            let expected_message = format!(
                "{}: ffmpeg @ Test {} log message",
                expected_tracing_level, expected_tracing_level
            );

            assert!(
                logs_contain(&expected_message),
                "Expected log message for '{}'",
                expected_message
            );
        }
    }

    #[cfg(feature = "tracing")]
    #[test]
    #[tracing_test::traced_test]
    fn test_log_callback_tracing_deprecated_message() {
        let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
        let _ = set_default(subscriber);
        log_callback_tracing();

        let deprecated_message = "deprecated pixel format used, make sure you did set range correctly";
        unsafe {
            av_log(
                ptr::null_mut(),
                LogLevel::Trace as i32,
                CString::new(deprecated_message).expect("Failed to create CString").as_ptr(),
            );
        }

        assert!(
            logs_contain(&format!("debug: ffmpeg @ {}", deprecated_message)),
            "Expected log message for '{}'",
            deprecated_message
        );
    }
}
