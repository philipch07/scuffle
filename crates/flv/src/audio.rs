use std::io;

use byteorder::ReadBytesExt;
use bytes::Bytes;
use scuffle_bytes_util::BytesCursorExt;

use super::aac::{AacPacket, AacPacketType};
use crate::macros::nutype_enum;

/// FLV Tag Audio Data
///
/// This is the container for the audio data.
///
/// Defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
/// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
#[derive(Debug, Clone, PartialEq)]
pub struct AudioData {
    /// The sound rate of the audio data. (2 bits)
    pub sound_rate: SoundRate,
    /// The sound size of the audio data. (1 bit)
    pub sound_size: SoundSize,
    /// The sound type of the audio data. (1 bit)
    pub sound_type: SoundType,
    /// The body of the audio data.
    pub body: AudioDataBody,
}

impl AudioData {
    pub fn demux(reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        // SoundFormat is the first 4 bits of the byte
        let sound_format = SoundFormat::from(byte >> 4);
        // SoundRate is the next 2 bits of the byte
        let sound_rate = SoundRate::from((byte >> 2) & 0b11);
        // SoundSize is the next bit of the byte
        let sound_size = SoundSize::from((byte >> 1) & 0b1);
        // SoundType is the last bit of the byte
        let sound_type = SoundType::from(byte & 0b1);

        // Now we can demux the body of the audio data
        let body = AudioDataBody::demux(sound_format, reader)?;

        Ok(AudioData {
            sound_rate,
            sound_size,
            sound_type,
            body,
        })
    }
}

nutype_enum! {
    /// FLV Sound Format
    ///
    /// Denotes the type of the underlying data packet
    ///
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
    pub enum SoundFormat(u8) {
        /// Linear PCM, platform endian
        LinearPcmPlatformEndian = 0,
        /// ADPCM
        Adpcm = 1,
        /// MP3
        Mp3 = 2,
        /// Linear PCM, little endian
        LinearPcmLittleEndian = 3,
        /// Nellymoser 16Khz Mono
        Nellymoser16KhzMono = 4,
        /// Nellymoser 8Khz Mono
        Nellymoser8KhzMono = 5,
        /// Nellymoser
        Nellymoser = 6,
        /// G.711 A-Law logarithmic PCM
        G711ALaw = 7,
        /// G.711 Mu-Law logarithmic PCM
        G711MuLaw = 8,
        /// AAC
        Aac = 10,
        /// Speex
        Speex = 11,
        /// Mp3 8Khz
        Mp38Khz = 14,
        /// Device specific sound
        DeviceSpecificSound = 15,
    }
}

/// FLV Tag Audio Data Body
///
/// This is the container for the audio data body.
///
/// Defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
/// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
#[derive(Debug, Clone, PartialEq)]
pub enum AudioDataBody {
    /// AAC Audio Packet
    Aac(AacPacket),
    /// Some other audio format we don't know how to parse
    Unknown { sound_format: SoundFormat, data: Bytes },
}

impl AudioDataBody {
    pub fn demux(sound_format: SoundFormat, reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        match sound_format {
            SoundFormat::Aac => {
                // For some reason the spec adds a specific byte before the AAC data.
                // This byte is the AAC packet type.
                let aac_packet_type = AacPacketType::from(reader.read_u8()?);
                Ok(Self::Aac(AacPacket::demux(aac_packet_type, reader)?))
            }
            _ => Ok(Self::Unknown {
                sound_format,
                data: reader.extract_remaining(),
            }),
        }
    }
}

nutype_enum! {
    /// FLV Sound Rate
    ///
    /// Denotes the sampling rate of the audio data.
    ///
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
    pub enum SoundRate(u8) {
        /// 5.5 KHz
        Hz5500 = 0,
        /// 11 KHz
        Hz11000 = 1,
        /// 22 KHz
        Hz22000 = 2,
        /// 44 KHz
        Hz44000 = 3,
    }
}

nutype_enum! {
    /// FLV Sound Size
    ///
    /// Denotes the size of each sample in the audio data.
    ///
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
    pub enum SoundSize(u8) {
        /// 8 bit
        Bit8 = 0,
        /// 16 bit
        Bit16 = 1,
    }
}

nutype_enum! {
    /// FLV Sound Type
    ///
    /// Denotes the number of channels in the audio data.
    ///
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Audio tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.2.1 - AUDIODATA)
    pub enum SoundType(u8) {
        /// Mono
        Mono = 0,
        /// Stereo
        Stereo = 1,
    }
}
