use ffmpeg_sys_next::*;

use super::internal::{seek, write_packet, Inner, InnerOptions};
use crate::consts::DEFAULT_BUFFER_SIZE;
use crate::dict::Dictionary;
use crate::error::FfmpegError;
use crate::packet::Packet;
use crate::stream::Stream;

#[derive(Debug, Clone)]
pub struct OutputOptions<'a> {
    pub buffer_size: usize,
    pub format_name: Option<&'a str>,
    pub format_mime_type: Option<&'a str>,
    pub format_ffi: *const AVOutputFormat,
}

impl<'a> OutputOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn format_name(mut self, format_name: &'a str) -> Self {
        self.format_name = format_name.into();
        self
    }

    pub fn format_mime_type(mut self, format_mime_type: &'a str) -> Self {
        self.format_mime_type = format_mime_type.into();
        self
    }

    pub fn format_ffi(mut self, format_ffi: *const AVOutputFormat) -> Self {
        self.format_ffi = format_ffi;
        self
    }

    fn get_format_ffi(&self) -> Result<*const AVOutputFormat, FfmpegError> {
        if !self.format_ffi.is_null() {
            return Ok(self.format_ffi);
        }

        if self.format_name.is_none() && self.format_mime_type.is_none() {
            return Err(FfmpegError::Arguments(
                "format_ffi, format_name and format_mime_type cannot all be unset",
            ));
        }

        let c_format_name = self.format_name.map(|s| std::ffi::CString::new(s).unwrap());
        let c_format_mime_type = self.format_mime_type.map(|s| std::ffi::CString::new(s).unwrap());
        let c_format_name_ptr = c_format_name.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
        let c_format_mime_type_ptr = c_format_mime_type.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());

        // Safety: `av_guess_format` is safe to call with null pointers.
        let output_format = unsafe { av_guess_format(c_format_name_ptr, std::ptr::null(), c_format_mime_type_ptr) };

        if output_format.is_null() {
            return Err(FfmpegError::Arguments("could not determine output format"));
        }

        Ok(output_format)
    }
}

impl Default for OutputOptions<'_> {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            format_name: None,
            format_mime_type: None,
            format_ffi: std::ptr::null(),
        }
    }
}

pub struct Output<T: Send + Sync> {
    inner: Inner<T>,
    witten_header: bool,
}

/// Safety: `T` must be `Send` and `Sync`.
unsafe impl<T: Send + Sync> Send for Output<T> {}

impl<T: Send + Sync> Output<T> {
    pub fn into_inner(mut self) -> T {
        *(self.inner.data.take().unwrap())
    }
}

impl<T: std::io::Write + Send + Sync> Output<T> {
    pub fn new(input: T, options: OutputOptions) -> Result<Self, FfmpegError> {
        let output_format = options.get_format_ffi()?;

        Ok(Self {
            inner: Inner::new(
                input,
                InnerOptions {
                    buffer_size: options.buffer_size,
                    write_fn: Some(write_packet::<T>),
                    output_format,
                    ..Default::default()
                },
            )?,
            witten_header: false,
        })
    }

    pub fn seekable(input: T, options: OutputOptions) -> Result<Self, FfmpegError>
    where
        T: std::io::Seek,
    {
        let output_format = options.get_format_ffi()?;

        Ok(Self {
            inner: Inner::new(
                input,
                InnerOptions {
                    buffer_size: options.buffer_size,
                    write_fn: Some(write_packet::<T>),
                    seek_fn: Some(seek::<T>),
                    output_format,
                    ..Default::default()
                },
            )?,
            witten_header: false,
        })
    }
}

impl<T: Send + Sync> Output<T> {
    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            // This frees the old metadata
            Dictionary::from_ptr_owned(self.inner.context.as_deref_mut_except().metadata);
            self.inner.context.as_deref_mut_except().metadata = metadata.into_ptr();
        };
    }

    pub fn as_ptr(&self) -> *const AVFormatContext {
        self.inner.context.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.inner.context.as_mut_ptr()
    }

    pub fn add_stream(&mut self, codec: Option<*const AVCodec>) -> Option<Stream<'_>> {
        // Safety: `avformat_new_stream` is safe to call.
        let stream = unsafe { avformat_new_stream(self.as_mut_ptr(), codec.unwrap_or_else(std::ptr::null)) };
        if stream.is_null() {
            None
        } else {
            // Safety: 'stream' is a valid pointer here.
            unsafe {
                let stream = &mut *stream;
                stream.id = self.inner.context.as_deref_except().nb_streams as i32 - 1;
                Some(Stream::new(stream, self.inner.context.as_deref_except()))
            }
        }
    }

    pub fn copy_stream<'a>(&'a mut self, stream: &Stream<'_>) -> Option<Stream<'a>> {
        let codec_param = stream.codec_parameters()?;

        // Safety: `avformat_new_stream` is safe to call.
        let out_stream = unsafe { avformat_new_stream(self.as_mut_ptr(), std::ptr::null()) };
        if out_stream.is_null() {
            None
        } else {
            // Safety: 'out_stream', 'codec_param' and 'context' are valid pointers here.
            unsafe {
                let out_stream = &mut *out_stream;

                // Safety: `avcodec_parameters_copy` is safe to call.
                avcodec_parameters_copy(out_stream.codecpar, codec_param);
                out_stream.id = self.inner.context.as_deref_except().nb_streams as i32 - 1;
                let mut out_stream = Stream::new(out_stream, self.inner.context.as_deref_except());
                out_stream.set_time_base(stream.time_base());
                out_stream.set_start_time(stream.start_time());
                out_stream.set_duration(stream.duration());

                Some(out_stream)
            }
        }
    }

    pub fn write_header(&mut self) -> Result<(), FfmpegError> {
        if self.witten_header {
            return Err(FfmpegError::Arguments("header already written"));
        }

        // Safety: `avformat_write_header` is safe to call, if the header has not been
        // written yet.
        unsafe {
            match avformat_write_header(self.as_mut_ptr(), std::ptr::null_mut()) {
                0 => Ok(()),
                e => Err(FfmpegError::Code(e.into())),
            }
        }?;

        self.witten_header = true;
        Ok(())
    }

    pub fn write_header_with_options(&mut self, options: &mut Dictionary) -> Result<(), FfmpegError> {
        if self.witten_header {
            return Err(FfmpegError::Arguments("header already written"));
        }

        // Safety: `avformat_write_header` is safe to call, if the header has not been
        // written yet.
        unsafe {
            match avformat_write_header(self.as_mut_ptr(), options.as_mut_ptr_ref()) {
                0 => Ok(()),
                e => Err(FfmpegError::Code(e.into())),
            }
        }?;

        self.witten_header = true;
        Ok(())
    }

    pub fn write_trailer(&mut self) -> Result<(), FfmpegError> {
        if !self.witten_header {
            return Err(FfmpegError::Arguments("header not written"));
        }

        // Safety: `av_write_trailer` is safe to call, once the header has been written.
        unsafe {
            match av_write_trailer(self.as_mut_ptr()) {
                n if n >= 0 => Ok(()),
                e => Err(FfmpegError::Code(e.into())),
            }
        }
    }

    pub fn write_interleaved_packet(&mut self, mut packet: Packet) -> Result<(), FfmpegError> {
        if !self.witten_header {
            return Err(FfmpegError::Arguments("header not written"));
        }

        // Safety: `av_interleaved_write_frame` is safe to call, once the header has
        // been written.
        unsafe {
            match av_interleaved_write_frame(self.as_mut_ptr(), packet.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(FfmpegError::Code(e.into())),
            }
        }
    }

    pub fn write_packet(&mut self, packet: &Packet) -> Result<(), FfmpegError> {
        if !self.witten_header {
            return Err(FfmpegError::Arguments("header not written"));
        }

        // Safety: `av_write_frame` is safe to call, once the header has been written.
        unsafe {
            match av_write_frame(self.as_mut_ptr(), packet.as_ptr() as *mut _) {
                0 => Ok(()),
                e => Err(FfmpegError::Code(e.into())),
            }
        }
    }

    pub fn flags(&self) -> i32 {
        self.inner.context.as_deref_except().flags
    }
}

impl Output<()> {
    pub fn open(path: &str) -> Result<Self, FfmpegError> {
        Ok(Self {
            inner: Inner::open_output(path)?,
            witten_header: false,
        })
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::ffi::CString;
    use std::io::Cursor;
    use std::ptr;

    use tempfile::Builder;

    use crate::consts::DEFAULT_BUFFER_SIZE;
    use crate::dict::Dictionary;
    use crate::error::FfmpegError;
    use crate::io::output::{AVCodec, AVRational, AVFMT_FLAG_AUTO_BSF};
    use crate::io::{Output, OutputOptions};

    #[test]
    fn test_output_options_default() {
        let options = OutputOptions::default();

        assert_eq!(options.buffer_size, DEFAULT_BUFFER_SIZE);
        assert!(options.format_name.is_none());
        assert!(options.format_mime_type.is_none());
        assert!(options.format_ffi.is_null());
    }

    #[test]
    fn test_output_options_new() {
        let options = OutputOptions::new();

        assert_eq!(options.buffer_size, DEFAULT_BUFFER_SIZE);
        assert!(options.format_name.is_none());
        assert!(options.format_mime_type.is_none());
        assert!(options.format_ffi.is_null());
    }

    #[test]
    fn test_output_options_custom_values() {
        let options = OutputOptions::default()
            .buffer_size(4096)
            .format_name("mp4")
            .format_mime_type("video/mp4");

        assert_eq!(options.buffer_size, 4096);
        assert_eq!(options.format_name, Some("mp4"));
        assert_eq!(options.format_mime_type, Some("video/mp4"));
    }

    #[test]
    fn test_output_options_get_format_ffi() {
        let options = OutputOptions::default().format_name("mp4");
        let format_ffi = options.get_format_ffi();
        assert!(format_ffi.is_ok());
        assert!(!format_ffi.unwrap().is_null());
    }

    #[test]
    fn test_output_options_get_format_ffi_null() {
        let format_name = CString::new("mp4").unwrap();
        let format_mime_type = CString::new("").unwrap();
        let format_ptr = unsafe { ffmpeg_sys_next::av_guess_format(format_name.as_ptr(), ptr::null(), format_mime_type.as_ptr()) };

        assert!(!format_ptr.is_null(), "Failed to retrieve AVOutputFormat for the given format name");

        let output_options = OutputOptions::new().format_ffi(format_ptr);
        let result = output_options.get_format_ffi();

        assert!(result.is_ok(), "Expected Ok result, got Err instead");
        assert_eq!(
            result.unwrap(),
            format_ptr,
            "Expected format_ffi pointer to match the retrieved format"
        );
    }

    #[test]
    fn test_output_options_get_format_ffi_unset_error() {
        let options = OutputOptions::default();
        let format_ffi = options.get_format_ffi();
        assert!(format_ffi.is_err());
    }

    #[test]
    fn test_output_options_get_format_ffi_output_format_error() {
        let options = OutputOptions::default().format_name("unknown_format");
        let format_ffi_result = options.get_format_ffi();

        assert!(format_ffi_result.is_err());
        match format_ffi_result {
            Err(FfmpegError::Arguments(msg)) => {
                assert_eq!(msg, "could not determine output format");
            }
            _ => panic!("Expected FfmpegError::Arguments"),
        }
    }

    #[test]
    fn test_output_into_inner() {
        let data = Cursor::new(Vec::with_capacity(1024));
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data, options).expect("Failed to create Output");
        let inner_data = output.into_inner();

        assert!(inner_data.get_ref().is_empty());
        let buffer = inner_data.into_inner();
        assert_eq!(buffer.capacity(), 1024);
    }

    #[test]
    fn test_output_new() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data, options);

        assert!(output.is_ok());
    }

    #[test]
    fn test_output_seekable() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::seekable(data, options);

        assert!(output.is_ok());
    }

    #[test]
    fn test_output_set_metadata() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).unwrap();
        let metadata = Dictionary::new();
        output.set_metadata(metadata);

        assert!(!output.as_ptr().is_null());
    }

    #[test]
    fn test_output_as_mut_ptr() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let context_ptr = output.as_mut_ptr();

        assert!(!context_ptr.is_null(), "Expected non-null pointer from as_mut_ptr");
    }

    #[test]
    fn test_add_stream_with_valid_codec() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");
        let dummy_codec: *const AVCodec = 0x1234 as *const AVCodec;
        let stream = output.add_stream(Some(dummy_codec));

        assert!(stream.is_some(), "Expected a valid Stream to be added");
    }

    #[test]
    fn test_copy_stream() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output = Output::new(data, options).expect("Failed to create Output");

        // create new output to prevent double mut borrow
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let mut output_two = Output::new(data, options).expect("Failed to create Output");

        let dummy_codec: *const AVCodec = 0x1234 as *const AVCodec;
        let mut source_stream = output.add_stream(Some(dummy_codec)).expect("Failed to add source stream");

        source_stream.set_time_base(AVRational { num: 1, den: 25 });
        source_stream.set_start_time(Some(1000));
        source_stream.set_duration(Some(500));
        let copied_stream = output_two.copy_stream(&source_stream).expect("Failed to copy the stream");

        assert_eq!(copied_stream.index(), source_stream.index(), "Stream indices should match");
        assert_eq!(copied_stream.id(), source_stream.id(), "Stream IDs should match");
        assert_eq!(
            copied_stream.time_base(),
            source_stream.time_base(),
            "Time bases should match"
        );
        assert_eq!(
            copied_stream.start_time(),
            source_stream.start_time(),
            "Start times should match"
        );
        assert_eq!(copied_stream.duration(), source_stream.duration(), "Durations should match");
        assert_eq!(
            copied_stream.duration(),
            source_stream.duration(),
            "Durations should match"
        );
        assert!(!copied_stream.as_ptr().is_null(), "Copied stream pointer should not be null");
    }

    #[test]
    fn test_output_flags() {
        let data = Cursor::new(Vec::new());
        let options = OutputOptions::default().format_name("mp4");
        let output = Output::new(data, options).expect("Failed to create Output");
        let flags = output.flags();

        assert_eq!(flags, AVFMT_FLAG_AUTO_BSF, "Expected default flag to be AVFMT_FLAG_AUTO_BSF");
    }

    #[test]
    fn test_output_open() {
        let temp_file = Builder::new()
            .suffix(".mp4")
            .tempfile()
            .expect("Failed to create a temporary file");
        let temp_path = temp_file.path();
        let output = Output::open(temp_path.to_str().unwrap());

        assert!(output.is_ok(), "Expected Output::open to succeed");
        std::fs::remove_file(temp_path).expect("Failed to remove temporary file");
    }
}
