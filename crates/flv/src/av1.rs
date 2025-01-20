use bytes::Bytes;
use scuffle_av1::AV1CodecConfigurationRecord;

/// AV1 Packet
/// This is a container for av1 data.
/// This enum contains the data for the different types of av1 packets.
#[derive(Debug, Clone, PartialEq)]
pub enum Av1Packet {
    /// AV1 Sequence Start
    SequenceStart(AV1CodecConfigurationRecord),
    /// AV1 Raw Data
    Raw(Bytes),
}
