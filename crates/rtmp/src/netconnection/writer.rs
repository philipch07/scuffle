use std::io;

use bytes::Bytes;
use scuffle_amf0::{Amf0Encoder, Amf0Value};

use super::errors::NetConnectionError;
use crate::chunk::{Chunk, ChunkEncoder, DefinedChunkStreamID};
use crate::messages::MessageTypeID;

pub struct NetConnection;

impl NetConnection {
    fn write_chunk(encoder: &ChunkEncoder, amf0: Bytes, writer: &mut impl io::Write) -> Result<(), NetConnectionError> {
        encoder.write_chunk(
            writer,
            Chunk::new(DefinedChunkStreamID::Command as u32, 0, MessageTypeID::CommandAMF0, 0, amf0),
        )?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn write_connect_response(
        encoder: &ChunkEncoder,
        writer: &mut impl io::Write,
        transaction_id: f64,
        fmsver: &str,
        capabilities: f64,
        code: &str,
        level: &str,
        description: &str,
        encoding: f64,
    ) -> Result<(), NetConnectionError> {
        let mut amf0_writer = Vec::new();

        Amf0Encoder::encode_string(&mut amf0_writer, "_result")?;
        Amf0Encoder::encode_number(&mut amf0_writer, transaction_id)?;
        Amf0Encoder::encode_object(
            &mut amf0_writer,
            &[
                ("fmsVer".into(), Amf0Value::String(fmsver.into())),
                ("capabilities".into(), Amf0Value::Number(capabilities)),
            ],
        )?;
        Amf0Encoder::encode_object(
            &mut amf0_writer,
            &[
                ("level".into(), Amf0Value::String(level.into())),
                ("code".into(), Amf0Value::String(code.into())),
                ("description".into(), Amf0Value::String(description.into())),
                ("objectEncoding".into(), Amf0Value::Number(encoding)),
            ],
        )?;

        Self::write_chunk(encoder, Bytes::from(amf0_writer), writer)
    }

    pub fn write_create_stream_response(
        encoder: &ChunkEncoder,
        writer: &mut impl io::Write,
        transaction_id: f64,
        stream_id: f64,
    ) -> Result<(), NetConnectionError> {
        let mut amf0_writer = Vec::new();

        Amf0Encoder::encode_string(&mut amf0_writer, "_result")?;
        Amf0Encoder::encode_number(&mut amf0_writer, transaction_id)?;
        Amf0Encoder::encode_null(&mut amf0_writer)?;
        Amf0Encoder::encode_number(&mut amf0_writer, stream_id)?;

        Self::write_chunk(encoder, Bytes::from(amf0_writer), writer)
    }
}
