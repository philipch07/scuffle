use std::io;

use bytes::Bytes;
use scuffle_amf0::{Amf0Decoder, Amf0Marker, Amf0Value};
use scuffle_bytes_util::BytesCursorExt;

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptData {
    /// The name of the script data
    pub name: String,
    /// The data of the script data
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

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_script_data() {
        let mut reader = io::Cursor::new(Bytes::from_static(&[
            0x02, // String marker
            0x00, 0x0A, // Length (10 bytes)
            b'o', b'n', b'M', b'e', b't', b'a', b'D', b'a', b't', b'a', // "onMetaData"
            0x05, // null marker
            0x05, // null marker

        ]));
        let script_data = ScriptData::demux(&mut reader).unwrap();
        assert_eq!(script_data.name, "onMetaData");
        assert_eq!(script_data.data.len(), 2);
        assert_eq!(script_data.data[0], Amf0Value::Null);
        assert_eq!(script_data.data[1], Amf0Value::Null);
    }
}
