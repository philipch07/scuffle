use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use h265::HEVCDecoderConfigurationRecord;
use scuffle_av1::{AV1CodecConfigurationRecord, AV1VideoDescriptor};
use scuffle_bytes_util::BytesCursorExt;

use super::av1::Av1Packet;
use super::avc::{AvcPacket, AvcPacketType};
use super::hevc::HevcPacket;
use crate::macros::nutype_enum;

nutype_enum! {
    /// FLV Frame Type
    /// This enum represents the different types of frames in a FLV file.
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Video tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.3.1 - VIDEODATA)
    pub enum FrameType(u8) {
        /// A keyframe is a frame that is a complete representation of the video content.
        Keyframe = 1,
        /// An interframe is a frame that is a partial representation of the video content.
        Interframe = 2,
        /// A disposable interframe is a frame that is a partial representation of the video content, but is not required to be displayed. (h263 only)
        DisposableInterframe = 3,
        /// A generated keyframe is a frame that is a complete representation of the video content, but is not a keyframe. (reserved for server use only)
        GeneratedKeyframe = 4,
        /// A video info or command frame is a frame that contains video information or commands.
        /// If the frame is this type, the body will be a CommandPacket
        VideoInfoOrCommandFrame = 5,
    }
}

/// FLV Tag Video Data
/// This is a container for video data.
/// This enum contains the data for the different types of video tags.
/// Defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Video
///   tags)
/// - video_file_format_spec_v10_1.pdf (Annex E.4.3.1 - VIDEODATA)
#[derive(Debug, Clone, PartialEq)]
pub struct VideoData {
    /// The frame type of the video data. (4 bits)
    pub frame_type: FrameType,
    /// The body of the video data.
    pub body: VideoDataBody,
}

impl VideoData {
    pub fn demux(reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        let enhanced = (byte & 0b1000_0000) != 0;
        let frame_type_byte = (byte >> 4) & 0b0111;
        let packet_type_byte = byte & 0b0000_1111;
        let frame_type = FrameType::from(frame_type_byte);
        let body = if frame_type == FrameType::VideoInfoOrCommandFrame {
            let command_packet = CommandPacket::from(reader.read_u8()?);
            VideoDataBody::Command(command_packet)
        } else {
            VideoDataBody::demux(VideoPacketType::new(packet_type_byte, enhanced), reader)?
        };

        Ok(VideoData { frame_type, body })
    }
}

nutype_enum! {
    /// FLV Video Codec ID
    ///
    /// Denotes the different types of video codecs that can be used in a FLV file.
    /// This is a legacy enum for older codecs; for modern codecs, the [`EnhancedPacketType`] is used which uses a [`VideoFourCC`] identifier.
    ///
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Video tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.3.1 - VIDEODATA)
    pub enum VideoCodecId(u8) {
        /// Sorenson H.263
        SorensonH263 = 2,
        /// Screen Video
        ScreenVideo = 3,
        /// On2 VP6
        On2VP6 = 4,
        /// On2 VP6 with alpha channel
        On2VP6WithAlphaChannel = 5,
        /// Screen Video Version 2
        ScreenVideoVersion2 = 6,
        /// AVC (H.264)
        Avc = 7,
    }
}

/// FLV Tag Video Data Body
///
/// This is a container for video data.
/// This enum contains the data for the different types of video tags.
///
/// Defined by:
/// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Video
///   tags)
/// - video_file_format_spec_v10_1.pdf (Annex E.4.3.1 - VIDEODATA)
#[derive(Debug, Clone, PartialEq)]
pub enum VideoDataBody {
    /// AVC Video Packet (H.264)
    /// When [`VideoPacketType::CodecId`] is [`VideoCodecId::Avc`]
    Avc(AvcPacket),
    /// Enhanced Packet (AV1, H.265, etc.)
    /// When [`VideoPacketType::Enhanced`] is used
    Enhanced(EnhancedPacket),
    /// Command Frame (VideoInfo or Command)
    /// When [`FrameType::VideoInfoOrCommandFrame`] is used
    Command(CommandPacket),
    /// Data we don't know how to parse
    Unknown { codec_id: VideoCodecId, data: Bytes },
}

nutype_enum! {
    /// FLV Command Packet
    /// Defined by:
    /// - video_file_format_spec_v10.pdf (Chapter 1 - The FLV File Format - Video tags)
    /// - video_file_format_spec_v10_1.pdf (Annex E.4.3.1 - VIDEODATA)
    pub enum CommandPacket(u8) {
        /// Start of client seeking, when FrameType is 5
        StartOfClientSeeking = 1,
        /// End of client seeking, when FrameType is 5
        EndOfClientSeeking = 2,
    }
}

/// A wrapper enum for the different types of video packets that can be used in
/// a FLV file.
///
/// Used to construct a [`VideoDataBody`].
///
/// See:
/// - [`VideoCodecId`]
/// - [`EnhancedPacketType`]
/// - [`VideoDataBody`]
#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum VideoPacketType {
    /// Codec ID (legacy)
    CodecId(VideoCodecId),
    /// Enhanced (modern)
    Enhanced(EnhancedPacketType),
}

impl VideoPacketType {
    pub fn new(byte: u8, enhanced: bool) -> Self {
        if enhanced {
            Self::Enhanced(EnhancedPacketType::from(byte))
        } else {
            Self::CodecId(VideoCodecId::from(byte))
        }
    }
}

impl VideoDataBody {
    /// Demux a video packet from the given reader.
    /// The reader will consume all the data from the reader.
    pub fn demux(packet_type: VideoPacketType, reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        match packet_type {
            VideoPacketType::CodecId(codec_id) => match codec_id {
                VideoCodecId::Avc => {
                    let avc_packet_type = AvcPacketType::from(reader.read_u8()?);
                    Ok(VideoDataBody::Avc(AvcPacket::demux(avc_packet_type, reader)?))
                }
                _ => Ok(VideoDataBody::Unknown {
                    codec_id,
                    data: reader.extract_remaining(),
                }),
            },
            VideoPacketType::Enhanced(packet_type) => {
                let mut video_codec = [0; 4];
                reader.read_exact(&mut video_codec)?;
                let video_codec = VideoFourCC::from(video_codec);

                match packet_type {
                    EnhancedPacketType::SequenceEnd => return Ok(VideoDataBody::Enhanced(EnhancedPacket::SequenceEnd)),
                    EnhancedPacketType::Metadata => {
                        return Ok(VideoDataBody::Enhanced(EnhancedPacket::Metadata(reader.extract_remaining())))
                    }
                    _ => {}
                }

                match (video_codec, packet_type) {
                    (VideoFourCC::Av1, EnhancedPacketType::SequenceStart) => Ok(VideoDataBody::Enhanced(
                        EnhancedPacket::Av1(Av1Packet::SequenceStart(AV1CodecConfigurationRecord::demux(reader)?)),
                    )),
                    (VideoFourCC::Av1, EnhancedPacketType::Mpeg2SequenceStart) => {
                        Ok(VideoDataBody::Enhanced(EnhancedPacket::Av1(Av1Packet::SequenceStart(
                            AV1VideoDescriptor::demux(reader)?.codec_configuration_record,
                        ))))
                    }
                    (VideoFourCC::Av1, EnhancedPacketType::CodedFrames) => Ok(VideoDataBody::Enhanced(EnhancedPacket::Av1(
                        Av1Packet::Raw(reader.extract_remaining()),
                    ))),
                    (VideoFourCC::Hevc, EnhancedPacketType::SequenceStart) => Ok(VideoDataBody::Enhanced(
                        EnhancedPacket::Hevc(HevcPacket::SequenceStart(HEVCDecoderConfigurationRecord::demux(reader)?)),
                    )),
                    (VideoFourCC::Hevc, EnhancedPacketType::CodedFrames) => {
                        Ok(VideoDataBody::Enhanced(EnhancedPacket::Hevc(HevcPacket::Nalu {
                            composition_time: Some(reader.read_i24::<BigEndian>()?),
                            data: reader.extract_remaining(),
                        })))
                    }
                    (VideoFourCC::Hevc, EnhancedPacketType::CodedFramesX) => {
                        Ok(VideoDataBody::Enhanced(EnhancedPacket::Hevc(HevcPacket::Nalu {
                            composition_time: None,
                            data: reader.extract_remaining(),
                        })))
                    }
                    _ => Ok(VideoDataBody::Enhanced(EnhancedPacket::Unknown {
                        packet_type,
                        video_codec,
                        data: reader.extract_remaining(),
                    })),
                }
            }
        }
    }
}

/// An Enhanced FLV Packet
///
/// This is a container for enhanced video packets.
/// The enchanced spec adds modern codecs to the FLV file format.
///
/// Defined by:
/// - enhanced_rtmp-v1.pdf (Defining Additional Video Codecs)
/// - enhanced_rtmp-v2.pdf (Enhanced Video)
#[derive(Debug, Clone, PartialEq)]
pub enum EnhancedPacket {
    /// Metadata
    Metadata(Bytes),
    /// Sequence End
    SequenceEnd,
    /// Av1 Video Packet
    Av1(Av1Packet),
    /// Hevc (H.265) Video Packet
    Hevc(HevcPacket),
    /// We don't know how to parse it
    Unknown {
        packet_type: EnhancedPacketType,
        video_codec: VideoFourCC,
        data: Bytes,
    },
}

nutype_enum! {
    pub enum VideoFourCC([u8; 4]) {
        Av1 = *b"av01",
        Vp9 = *b"vp09",
        Hevc = *b"hvc1",
    }
}

nutype_enum! {
    pub enum EnhancedPacketType(u8) {
        SequenceStart = 0,
        CodedFrames = 1,
        SequenceEnd = 2,
        CodedFramesX = 3,
        Metadata = 4,
        Mpeg2SequenceStart = 5,
    }
}
