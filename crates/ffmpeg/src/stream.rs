use ffmpeg_sys_next::*;

use crate::consts::{Const, Mut};
use crate::dict::Dictionary;
use crate::utils::check_i64;

#[derive(Debug)]
pub struct Streams<'a> {
    input: &'a AVFormatContext,
}

/// Safety: `Streams` is safe to send between threads.
unsafe impl Send for Streams<'_> {}

impl<'a> Streams<'a> {
    pub(crate) fn new(input: &'a AVFormatContext) -> Self {
        Self { input }
    }

    pub fn best(&self, media_type: AVMediaType) -> Option<Const<'a, Stream<'a>>> {
        // Safety: av_find_best_stream is safe to call, 'input' is a valid pointer
        // We upcast the pointer to a mutable pointer because the function signature
        // requires it, but it does not mutate the pointer.
        let stream =
            unsafe { av_find_best_stream(self.input as *const _ as *mut _, media_type, -1, -1, std::ptr::null_mut(), 0) };
        if stream < 0 {
            return None;
        }

        // Safety: if we get back an index, it's valid
        let stream = unsafe { &mut *(*self.input.streams.add(stream as usize)) };

        Some(Const::new(Stream::new(stream, self.input)))
    }

    pub fn best_mut(&mut self, media_type: AVMediaType) -> Option<Stream<'a>> {
        self.best(media_type).map(|s| s.0)
    }
}

impl<'a> IntoIterator for Streams<'a> {
    type IntoIter = StreamIter<'a>;
    type Item = Const<'a, Stream<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        StreamIter {
            input: self.input,
            index: 0,
        }
    }
}

impl<'a> Streams<'a> {
    pub fn iter(&'a self) -> StreamIter<'a> {
        StreamIter {
            input: self.input,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.input.nb_streams as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&mut self, index: usize) -> Option<Stream<'_>> {
        if index >= self.len() {
            return None;
        }

        let stream = unsafe { &mut *(*self.input.streams.add(index)) };
        Some(Stream::new(stream, self.input))
    }
}

pub struct StreamIter<'a> {
    input: &'a AVFormatContext,
    index: usize,
}

impl<'a> Iterator for StreamIter<'a> {
    type Item = Const<'a, Stream<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.nb_streams as usize {
            return None;
        }

        let stream = unsafe { &mut *(*self.input.streams.add(self.index)) };
        self.index += 1;

        Some(Const::new(Stream::new(stream, self.input)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.input.nb_streams as usize - self.index;
        (remaining, Some(remaining))
    }
}

impl std::iter::ExactSizeIterator for StreamIter<'_> {}

pub struct Stream<'a>(&'a mut AVStream, &'a AVFormatContext);

impl<'a> Stream<'a> {
    pub(crate) fn new(stream: &'a mut AVStream, input: &'a AVFormatContext) -> Self {
        Self(stream, input)
    }

    pub fn as_ptr(&self) -> *const AVStream {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVStream {
        self.0
    }
}

impl<'a> Stream<'a> {
    pub fn index(&self) -> i32 {
        self.0.index
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn codec_parameters(&self) -> Option<&'a AVCodecParameters> {
        // Safety: the pointer is valid
        unsafe { self.0.codecpar.as_ref() }
    }

    pub fn time_base(&self) -> AVRational {
        self.0.time_base
    }

    pub fn set_time_base(&mut self, time_base: AVRational) {
        self.0.time_base = time_base;
    }

    pub fn start_time(&self) -> Option<i64> {
        check_i64(self.0.start_time)
    }

    pub fn set_start_time(&mut self, start_time: Option<i64>) {
        self.0.start_time = start_time.unwrap_or(AV_NOPTS_VALUE)
    }

    pub fn duration(&self) -> Option<i64> {
        check_i64(self.0.duration)
    }

    pub fn set_duration(&mut self, duration: Option<i64>) {
        self.0.duration = duration.unwrap_or(AV_NOPTS_VALUE)
    }

    pub fn nb_frames(&self) -> Option<i64> {
        check_i64(self.0.nb_frames)
    }

    pub fn set_nb_frames(&mut self, nb_frames: i64) {
        self.0.nb_frames = nb_frames;
    }

    pub fn disposition(&self) -> i32 {
        self.0.disposition
    }

    pub fn set_disposition(&mut self, disposition: i32) {
        self.0.disposition = disposition;
    }

    pub fn discard(&self) -> AVDiscard {
        self.0.discard
    }

    pub fn set_discard(&mut self, discard: AVDiscard) {
        self.0.discard = discard;
    }

    pub fn sample_aspect_ratio(&self) -> AVRational {
        self.0.sample_aspect_ratio
    }

    pub fn set_sample_aspect_ratio(&mut self, sample_aspect_ratio: AVRational) {
        self.0.sample_aspect_ratio = sample_aspect_ratio;
    }

    pub fn metadata(&self) -> Const<'_, Dictionary> {
        // Safety: the pointer metadata pointer does not live longer than this object,
        // see `Const::new`
        Const::new(unsafe { Dictionary::from_ptr_ref(self.0.metadata) })
    }

    pub fn metadata_mut(&mut self) -> Mut<'_, Dictionary> {
        // Safety: the pointer metadata pointer does not live longer than this object,
        // see `Mut::new`
        Mut::new(unsafe { Dictionary::from_ptr_ref(self.0.metadata) })
    }

    pub fn avg_frame_rate(&self) -> AVRational {
        self.0.avg_frame_rate
    }

    pub fn r_frame_rate(&self) -> AVRational {
        self.0.r_frame_rate
    }

    pub fn format_context(&self) -> &'a AVFormatContext {
        self.1
    }
}

impl std::fmt::Debug for Stream<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("index", &self.index())
            .field("id", &self.id())
            .field("time_base", &self.time_base())
            .field("start_time", &self.start_time())
            .field("duration", &self.duration())
            .field("nb_frames", &self.nb_frames())
            .field("disposition", &self.disposition())
            .field("discard", &self.discard())
            .field("sample_aspect_ratio", &self.sample_aspect_ratio())
            .field("metadata", &self.metadata())
            .field("avg_frame_rate", &self.avg_frame_rate())
            .field("r_frame_rate", &self.r_frame_rate())
            .finish()
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::collections::BTreeMap;

    use ffmpeg_sys_next::{AVDiscard, AVRational, AVStream};
    use insta::{assert_debug_snapshot, Settings};

    use crate::io::Input;
    use crate::stream::AVMediaType;

    #[test]
    fn test_best_stream() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();

        let media_type = AVMediaType::AVMEDIA_TYPE_VIDEO;
        let best_stream = streams.best(media_type);

        assert!(best_stream.is_some(), "Expected best stream to be found");
        let best_stream = best_stream.unwrap();
        assert!(best_stream.index() >= 0, "Expected a valid stream index");
    }

    #[test]
    fn test_best_none_stream() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams();
        let invalid_media_type = AVMediaType::AVMEDIA_TYPE_SUBTITLE;
        let best_stream = streams.best(invalid_media_type);

        assert!(
            best_stream.is_none(),
            "Expected `best` to return None for unsupported media type"
        );
    }

    #[test]
    fn test_best_mut_stream() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();

        let media_type = AVMediaType::AVMEDIA_TYPE_VIDEO;
        let best_mut_stream = streams.best_mut(media_type);

        assert!(best_mut_stream.is_some(), "Expected best mutable stream to be found");
        let best_mut_stream = best_mut_stream.unwrap();
        assert!(best_mut_stream.index() >= 0, "Expected a valid stream index");
    }

    #[test]
    fn test_streams_into_iter() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams_mut();
        let streams_len = streams.len();
        let iter = streams.into_iter();
        let collected_streams: Vec<_> = iter.collect();

        assert_eq!(
            collected_streams.len(),
            streams_len,
            "Expected the iterator to yield the same number of streams as `streams.len()`"
        );

        for stream in collected_streams {
            assert!(stream.index() >= 0, "Expected a valid stream index");
        }
    }

    #[test]
    fn test_streams_iter() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let streams = input.streams_mut();
        let iter = streams.iter();
        let collected_streams: Vec<_> = iter.collect();

        assert_eq!(
            collected_streams.len(),
            streams.len(),
            "Expected iterator to yield the same number of streams as `streams.len()`"
        );

        for stream in collected_streams {
            assert!(stream.index() >= 0, "Expected a valid stream index");
        }
    }

    #[test]
    fn test_streams_get_valid_index() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let stream_index = 0;
        let stream = streams.get(stream_index);

        assert!(stream.is_some(), "Expected `get` to return Some for a valid index");
        let stream = stream.unwrap();

        assert_eq!(stream.index(), stream_index as i32, "Stream index should match");
        assert!(stream.id() >= 0, "Stream ID should be valid");
    }

    #[test]
    fn test_streams_get_invalid_index() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let invalid_index = streams.len();
        let stream = streams.get(invalid_index);

        assert!(stream.is_none(), "Expected `get` to return None for an invalid index");
    }

    #[test]
    fn test_stream_as_mut_ptr() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let stream_index = 0;
        let mut stream = streams.get(stream_index).expect("Expected a valid stream");
        let stream_mut_ptr = stream.as_mut_ptr();

        assert!(!stream_mut_ptr.is_null(), "Expected a non-null mutable pointer");
        assert_eq!(
            stream_mut_ptr,
            stream.as_ptr() as *mut AVStream,
            "Mutable pointer should match the constant pointer cast to mutable"
        );
    }

    #[test]
    fn test_stream_nb_frames() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");

        let test_nb_frames = 100;
        stream.set_nb_frames(test_nb_frames);
        assert_eq!(
            stream.nb_frames(),
            Some(test_nb_frames),
            "Expected `nb_frames` to match the set value"
        );
    }

    #[test]
    fn test_stream_disposition() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");

        let test_disposition = 0x01;
        stream.set_disposition(test_disposition);
        assert_eq!(
            stream.disposition(),
            test_disposition,
            "Expected `disposition` to match the set value"
        );
    }

    #[test]
    fn test_stream_discard() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");

        let test_discard = AVDiscard::AVDISCARD_ALL;
        stream.set_discard(test_discard);
        assert_eq!(stream.discard(), test_discard, "Expected `discard` to match the set value");
    }

    #[test]
    fn test_stream_sample_aspect_ratio() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");

        let test_aspect_ratio = AVRational { num: 4, den: 3 };
        stream.set_sample_aspect_ratio(test_aspect_ratio);
        assert_eq!(
            stream.sample_aspect_ratio(),
            test_aspect_ratio,
            "Expected `sample_aspect_ratio` to match the set value"
        );
    }

    #[test]
    fn test_stream_metadata_insta() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let mut stream = streams.get(0).expect("Expected a valid stream");
        let mut metadata = stream.metadata_mut();
        let _ = metadata.set("test_key", "test_value");
        let _ = metadata.set("test_key_2", "test_value_2");
        let metadata = stream.metadata();

        // sorting metadata as the order is not guaranteed
        let sorted_metadata: BTreeMap<_, _> = metadata
            .iter()
            .filter_map(|(key, value)| {
                // convert `CStr` to `&str` to gracefully handle invalid UTF-8
                Some((key.to_str().ok()?.to_string(), value.to_str().ok()?.to_string()))
            })
            .collect();

        assert_debug_snapshot!(sorted_metadata, @r###"
        {
            "encoder": "Lavc60.9.100 libx264",
            "handler_name": "GPAC ISO Video Handler",
            "language": "und",
            "test_key": "test_value",
            "test_key_2": "test_value_2",
            "vendor_id": "[0][0][0][0]",
        }
        "###);
    }

    #[test]
    fn test_stream_frame_rates() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let stream = streams.get(0).expect("Expected a valid stream");
        let avg_frame_rate = stream.avg_frame_rate();
        let real_frame_rate = stream.r_frame_rate();

        assert!(avg_frame_rate.num > 0, "Expected non-zero avg_frame_rate numerator");
        assert!(real_frame_rate.num > 0, "Expected non-zero r_frame_rate numerator");
    }

    #[test]
    fn test_stream_format_context() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let stream = streams.get(0).expect("Expected a valid stream");

        let format_context = stream.format_context();
        assert_eq!(
            format_context as *const _,
            input.as_ptr(),
            "Expected `format_context` to match the input's context"
        );
    }

    #[test]
    fn test_stream_debug() {
        let valid_file_path = "../../assets/avc_aac_large.mp4";
        let mut input = Input::open(valid_file_path).expect("Failed to open valid file");
        let mut streams = input.streams_mut();
        let stream = streams.get(0).expect("Expected a valid stream");

        let metadata = stream.metadata();
        // sorting metadata as the order is not guaranteed
        let sorted_metadata: BTreeMap<_, _> = metadata
            .iter()
            .filter_map(|(key, value)| {
                // convert `CStr` to `&str` to gracefully handle invalid UTF-8
                Some((key.to_str().ok()?.to_string(), value.to_str().ok()?.to_string()))
            })
            .collect();

        let serialized_metadata = sorted_metadata
            .iter()
            .map(|(key, value)| format!("        \"{}\": \"{}\",", key, value))
            .collect::<Vec<_>>()
            .join("\n");

        let replacement_metadata = format!("metadata: {{\n{}\n    }}", serialized_metadata);
        let mut settings = Settings::new();
        let metadata_regex = r"metadata: \{[^}]*\}";
        settings.add_filter(metadata_regex, &replacement_metadata);

        settings.bind(|| {
            assert_debug_snapshot!(stream, @r#"
            Stream {
                index: 0,
                id: 1,
                time_base: AVRational {
                    num: 1,
                    den: 15360,
                },
                start_time: Some(
                    0,
                ),
                duration: Some(
                    16384,
                ),
                nb_frames: Some(
                    64,
                ),
                disposition: 1,
                discard: AVDISCARD_DEFAULT,
                sample_aspect_ratio: AVRational {
                    num: 1,
                    den: 1,
                },
                metadata: {
                    "encoder": "Lavc60.9.100 libx264",
                    "handler_name": "GPAC ISO Video Handler",
                    "language": "und",
                    "vendor_id": "[0][0][0][0]",
                },
                avg_frame_rate: AVRational {
                    num: 60,
                    den: 1,
                },
                r_frame_rate: AVRational {
                    num: 60,
                    den: 1,
                },
            }
            "#);
        });
    }
}
