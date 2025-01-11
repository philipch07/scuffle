use scuffle_av1::AV1CodecConfigurationRecord;
use bytes::Bytes;

/// AV1 Packet
/// This is a container for av1 data.
/// This enum contains the data for the different types of av1 packets.
#[derive(Debug, Clone, PartialEq)]
pub enum Av1Packet {
    SequenceStart(AV1CodecConfigurationRecord),
    Raw(Bytes),
}
