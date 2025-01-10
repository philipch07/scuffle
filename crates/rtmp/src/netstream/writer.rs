use std::io;

use bytes::Bytes;
use scuffle_amf0::{Amf0Encoder, Amf0Value};

use super::errors::NetStreamError;
use crate::chunk::{Chunk, ChunkEncoder, DefinedChunkStreamID};
use crate::messages::MessageTypeID;

pub struct NetStreamWriter {}

impl NetStreamWriter {
    fn write_chunk(encoder: &ChunkEncoder, amf0_writer: Bytes, writer: &mut impl io::Write) -> Result<(), NetStreamError> {
        encoder.write_chunk(
            writer,
            Chunk::new(
                DefinedChunkStreamID::Command as u32,
                0,
                MessageTypeID::CommandAMF0,
                0,
                amf0_writer,
            ),
        )?;

        Ok(())
    }

    pub fn write_on_status(
        encoder: &ChunkEncoder,
        writer: &mut impl io::Write,
        transaction_id: f64,
        level: &str,
        code: &str,
        description: &str,
    ) -> Result<(), NetStreamError> {
        let mut amf0_writer = Vec::new();

        Amf0Encoder::encode_string(&mut amf0_writer, "onStatus")?;
        Amf0Encoder::encode_number(&mut amf0_writer, transaction_id)?;
        Amf0Encoder::encode_null(&mut amf0_writer)?;
        Amf0Encoder::encode_object(
            &mut amf0_writer,
            &[
                ("level".into(), Amf0Value::String(level.into())),
                ("code".into(), Amf0Value::String(code.into())),
                ("description".into(), Amf0Value::String(description.into())),
            ],
        )?;

        Self::write_chunk(encoder, Bytes::from(amf0_writer), writer)
    }
}
