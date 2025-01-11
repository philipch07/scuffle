use bytes::Bytes;
use scuffle_bytes_util::BytesCursorExt;

use crate::macros::nutype_enum;

nutype_enum! {
    /// FLV AAC Packet Type
    ///
    /// Defined in the FLV specification. Chapter 1 - AACAUDIODATA
    ///
    /// The AACPacketType indicates the type of data in the AACAUDIODATA.
    pub enum AacPacketType(u8) {
        /// Sequence Header
        SequenceHeader = 0x0,
        /// Raw
        Raw = 0x1,
    }
}

/// AAC Packet
/// This is a container for aac data.
/// This enum contains the data for the different types of aac packets.
/// Defined in the FLV specification. Chapter 1 - AACAUDIODATA
#[derive(Debug, Clone, PartialEq)]
pub enum AacPacket {
    /// AAC Sequence Header
    SequenceHeader(Bytes),
    /// AAC Raw
    Raw(Bytes),
    /// Data we don't know how to parse
    Unknown { aac_packet_type: AacPacketType, data: Bytes },
}

impl AacPacket {
    pub fn demux(aac_packet_type: AacPacketType, reader: &mut std::io::Cursor<Bytes>) -> std::io::Result<Self> {
        let data = reader.extract_remaining();

        match aac_packet_type {
            AacPacketType::Raw => Ok(AacPacket::Raw(data)),
            AacPacketType::SequenceHeader => Ok(AacPacket::SequenceHeader(data)),
            _ => Ok(AacPacket::Unknown { aac_packet_type, data }),
        }
    }
}
