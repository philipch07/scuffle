use std::io;

use bytes::Bytes;
use scuffle_amf0::{Amf0Decoder, Amf0Marker, Amf0Value};
use scuffle_bytes_util::BytesCursorExt;

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptData {
    pub name: String,
    pub data: Vec<Amf0Value<'static>>,
}

impl ScriptData {
    pub fn demux(reader: &mut io::Cursor<Bytes>) -> io::Result<Self> {
        let buf = reader.extract_remaining();
        let mut amf0_reader = Amf0Decoder::new(&buf);

        let name = match amf0_reader.decode_with_type(Amf0Marker::String) {
            Ok(Amf0Value::String(name)) => name,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid script data name")),
        };

        let data = amf0_reader
            .decode_all()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid script data"))?;

        Ok(Self {
            name: name.into_owned(),
            data: data.into_iter().map(|v| v.to_owned()).collect(),
        })
    }
}
