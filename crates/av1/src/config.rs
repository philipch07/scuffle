use std::io;

use bytes::Bytes;
use scuffle_bytes_util::{BitReader, BitWriter, BytesCursorExt};

#[derive(Debug, Clone, PartialEq)]
/// AV1 Codec Configuration Record
/// https://aomediacodec.github.io/av1-isobmff/#av1codecconfigurationbox-syntax
pub struct AV1CodecConfigurationRecord {
    pub seq_profile: u8,
    pub seq_level_idx_0: u8,
    pub seq_tier_0: bool,
    pub high_bitdepth: bool,
    pub twelve_bit: bool,
    pub monochrome: bool,
    pub chroma_subsampling_x: bool,
    pub chroma_subsampling_y: bool,
    pub chroma_sample_position: u8,
    pub initial_presentation_delay_minus_one: Option<u8>,
    pub config_obu: Bytes,
}

impl AV1CodecConfigurationRecord {
    pub fn demux(reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        let mut bit_reader = BitReader::new(reader);

        let marker = bit_reader.read_bit()?;
        if !marker {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "marker is not set"));
        }

        let version = bit_reader.read_bits(7)? as u8;
        if version != 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "version is not 1"));
        }

        let seq_profile = bit_reader.read_bits(3)? as u8;
        let seq_level_idx_0 = bit_reader.read_bits(5)? as u8;

        let seq_tier_0 = bit_reader.read_bit()?;
        let high_bitdepth = bit_reader.read_bit()?;
        let twelve_bit = bit_reader.read_bit()?;
        let monochrome = bit_reader.read_bit()?;
        let chroma_subsampling_x = bit_reader.read_bit()?;
        let chroma_subsampling_y = bit_reader.read_bit()?;
        let chroma_sample_position = bit_reader.read_bits(2)? as u8;

        bit_reader.seek_bits(3)?; // reserved 3 bits

        let initial_presentation_delay_minus_one = if bit_reader.read_bit()? {
            Some(bit_reader.read_bits(4)? as u8)
        } else {
            bit_reader.seek_bits(4)?; // reserved 4 bits
            None
        };

        if !bit_reader.is_aligned() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Bit reader is not aligned"));
        }

        let reader = bit_reader.into_inner();

        Ok(AV1CodecConfigurationRecord {
            seq_profile,
            seq_level_idx_0,
            seq_tier_0,
            high_bitdepth,
            twelve_bit,
            monochrome,
            chroma_subsampling_x,
            chroma_subsampling_y,
            chroma_sample_position,
            initial_presentation_delay_minus_one,
            config_obu: reader.extract_remaining(),
        })
    }

    pub fn size(&self) -> u64 {
        1 // marker, version
        + 1 // seq_profile, seq_level_idx_0
        + 1 // seq_tier_0, high_bitdepth, twelve_bit, monochrome, chroma_subsampling_x, chroma_subsampling_y, chroma_sample_position
        + 1 // reserved, initial_presentation_delay_present, initial_presentation_delay_minus_one/reserved
        + self.config_obu.len() as u64
    }

    pub fn mux<T: io::Write>(&self, writer: &mut T) -> io::Result<()> {
        let mut bit_writer = BitWriter::new(writer);

        bit_writer.write_bit(true)?; // marker
        bit_writer.write_bits(1, 7)?; // version

        bit_writer.write_bits(self.seq_profile as u64, 3)?;
        bit_writer.write_bits(self.seq_level_idx_0 as u64, 5)?;

        bit_writer.write_bit(self.seq_tier_0)?;
        bit_writer.write_bit(self.high_bitdepth)?;
        bit_writer.write_bit(self.twelve_bit)?;
        bit_writer.write_bit(self.monochrome)?;
        bit_writer.write_bit(self.chroma_subsampling_x)?;
        bit_writer.write_bit(self.chroma_subsampling_y)?;
        bit_writer.write_bits(self.chroma_sample_position as u64, 2)?;

        bit_writer.write_bits(0, 3)?; // reserved 3 bits

        if let Some(initial_presentation_delay_minus_one) = self.initial_presentation_delay_minus_one {
            bit_writer.write_bit(true)?;
            bit_writer.write_bits(initial_presentation_delay_minus_one as u64, 4)?;
        } else {
            bit_writer.write_bit(false)?;
            bit_writer.write_bits(0, 4)?; // reserved 4 bits
        }

        bit_writer.finish()?.write_all(&self.config_obu)?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {

    use super::*;

    #[test]
    fn test_config_demux() {
        let data = b"\x81\r\x0c\0\n\x0f\0\0\0j\xef\xbf\xe1\xbc\x02\x19\x90\x10\x10\x10@".to_vec();

        let config = AV1CodecConfigurationRecord::demux(&mut io::Cursor::new(data.into())).unwrap();

        insta::assert_debug_snapshot!(config, @r#"
        AV1CodecConfigurationRecord {
            seq_profile: 0,
            seq_level_idx_0: 13,
            seq_tier_0: false,
            high_bitdepth: false,
            twelve_bit: false,
            monochrome: false,
            chroma_subsampling_x: true,
            chroma_subsampling_y: true,
            chroma_sample_position: 0,
            initial_presentation_delay_minus_one: None,
            config_obu: b"\n\x0f\0\0\0j\xef\xbf\xe1\xbc\x02\x19\x90\x10\x10\x10@",
        }
        "#);
    }

    #[test]
    fn test_marker_is_not_set() {
        let data = vec![0b00000000];

        let err = AV1CodecConfigurationRecord::demux(&mut io::Cursor::new(data.into())).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "marker is not set");
    }

    #[test]
    fn test_version_is_not_1() {
        let data = vec![0b10000000];

        let err = AV1CodecConfigurationRecord::demux(&mut io::Cursor::new(data.into())).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "version is not 1");
    }

    #[test]
    fn test_config_demux_with_initial_presentation_delay() {
        let data = b"\x81\r\x0c\x3f\n\x0f\0\0\0j\xef\xbf\xe1\xbc\x02\x19\x90\x10\x10\x10@".to_vec();

        let config = AV1CodecConfigurationRecord::demux(&mut io::Cursor::new(data.into())).unwrap();

        insta::assert_debug_snapshot!(config, @r#"
        AV1CodecConfigurationRecord {
            seq_profile: 0,
            seq_level_idx_0: 13,
            seq_tier_0: false,
            high_bitdepth: false,
            twelve_bit: false,
            monochrome: false,
            chroma_subsampling_x: true,
            chroma_subsampling_y: true,
            chroma_sample_position: 0,
            initial_presentation_delay_minus_one: Some(
                15,
            ),
            config_obu: b"\n\x0f\0\0\0j\xef\xbf\xe1\xbc\x02\x19\x90\x10\x10\x10@",
        }
        "#);
    }

    #[test]
    fn test_config_mux() {
        let config = AV1CodecConfigurationRecord {
            seq_profile: 0,
            seq_level_idx_0: 0,
            seq_tier_0: false,
            high_bitdepth: false,
            twelve_bit: false,
            monochrome: false,
            chroma_subsampling_x: false,
            chroma_subsampling_y: false,
            chroma_sample_position: 0,
            initial_presentation_delay_minus_one: None,
            config_obu: Bytes::from_static(b"HELLO FROM THE OBU"),
        };

        let mut buf = Vec::new();
        config.mux(&mut buf).unwrap();

        insta::assert_snapshot!(format!("{:?}", Bytes::from(buf)), @r#"b"\x81\0\0\0HELLO FROM THE OBU""#);
    }

    #[test]
    fn test_config_mux_with_delay() {
        let config = AV1CodecConfigurationRecord {
            seq_profile: 0,
            seq_level_idx_0: 0,
            seq_tier_0: false,
            high_bitdepth: false,
            twelve_bit: false,
            monochrome: false,
            chroma_subsampling_x: false,
            chroma_subsampling_y: false,
            chroma_sample_position: 0,
            initial_presentation_delay_minus_one: Some(0),
            config_obu: Bytes::from_static(b"HELLO FROM THE OBU"),
        };

        let mut buf = Vec::new();
        config.mux(&mut buf).unwrap();

        insta::assert_snapshot!(format!("{:?}", Bytes::from(buf)), @r#"b"\x81\0\0\x10HELLO FROM THE OBU""#);
    }
}
