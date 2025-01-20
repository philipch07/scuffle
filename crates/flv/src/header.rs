use std::io;

use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use scuffle_bytes_util::BytesCursorExt;

/// The FLV Header
/// Whenever a FLV file is read these are the first 9 bytes of the file.
///
/// Defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV Header - Page 8)
/// - video_file_format_spec_v10_1.pdf (Annex E.2 - The FLV Header)
#[derive(Debug, Clone, PartialEq)]
pub struct FlvHeader {
    /// The version of the FLV file.
    pub version: u8,
    /// Whether the FLV file has audio.
    pub has_audio: bool,
    /// Whether the FLV file has video.
    pub has_video: bool,
    /// The extra data in the FLV file.
    /// Since the header provides a data offset, this is the bytes between the
    /// end of the header and the start of the data.
    pub extra: Bytes,
}

impl FlvHeader {
    /// Demux the FLV header from the given reader.
    /// The reader will be returned in the position of the start of the data
    /// offset.
    pub fn demux(reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        let start = reader.position() as usize;

        let signature = reader.read_u24::<BigEndian>()?;

        // 0 byte at the beginning because we are only reading 3 bytes not 4.
        if signature != u32::from_be_bytes([0, b'F', b'L', b'V']) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid signature"));
        }

        let version = reader.read_u8()?;
        let flags = reader.read_u8()?;
        let has_audio = (flags & 0b00000100) != 0;
        let has_video = (flags & 0b00000001) != 0;

        let offset = reader.read_u32::<BigEndian>()? as usize;
        let end = reader.position() as usize;
        let size = end - start;

        let extra = reader.extract_bytes(
            offset
                .checked_sub(size)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid offset"))?,
        )?;

        Ok(FlvHeader {
            version,
            has_audio,
            has_video,
            extra,
        })
    }
}
