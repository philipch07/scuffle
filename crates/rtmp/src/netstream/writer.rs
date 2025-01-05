use std::collections::HashMap;
use std::io;

use amf0::{Amf0Value, Amf0Writer};
use bytes::Bytes;

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

        Amf0Writer::write_string(&mut amf0_writer, "onStatus")?;
        Amf0Writer::write_number(&mut amf0_writer, transaction_id)?;
        Amf0Writer::write_null(&mut amf0_writer)?;
        Amf0Writer::write_object(
            &mut amf0_writer,
            &HashMap::from([
                ("level".to_string(), Amf0Value::String(level.to_string())),
                ("code".to_string(), Amf0Value::String(code.to_string())),
                ("description".to_string(), Amf0Value::String(description.to_string())),
            ]),
        )?;

        Self::write_chunk(encoder, Bytes::from(amf0_writer), writer)
    }
}
