use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, Bytes};

use super::header::FlvHeader;
use super::tag::FlvTag;

/// An FLV file is a combination of a [`FlvHeader`] followed by the
/// `FLVFileBody` (which is just a series of [`FlvTag`]s)
///
/// The `FLVFileBody` is defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Page 8)
/// - video_file_format_spec_v10_1.pdf (Annex E.3 - The FLV File Body)
#[derive(Debug, Clone, PartialEq)]
pub struct FlvFile {
    pub header: FlvHeader,
    pub tags: Vec<FlvTag>,
}

impl FlvFile {
    pub fn demux(reader: &mut std::io::Cursor<Bytes>) -> std::io::Result<Self> {
        let header = FlvHeader::demux(reader)?;

        let mut tags = Vec::new();
        while reader.has_remaining() {
            // We don't care about the previous tag size, its only really used for seeking
            // backwards.
            reader.read_u32::<BigEndian>()?;

            // If there is no more data, we can stop reading.
            if !reader.has_remaining() {
                break;
            }

            // Demux the tag from the reader.
            let tag = FlvTag::demux(reader)?;
            tags.push(tag);
        }

        Ok(FlvFile { header, tags })
    }
}
