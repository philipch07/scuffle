use std::borrow::Cow;

use bytes::{BufMut, BytesMut};
use scuffle_amf0::{Amf0Decoder, Amf0Value, Amf0WriteError};

use super::NetConnection;
use crate::chunk::{ChunkDecoder, ChunkEncodeError, ChunkEncoder};
use crate::netconnection::NetConnectionError;

#[test]
fn test_error_display() {
    let error = NetConnectionError::Amf0Write(Amf0WriteError::NormalStringTooLong);
    assert_eq!(error.to_string(), "amf0 write error: normal string too long");

    let error = NetConnectionError::ChunkEncode(ChunkEncodeError::UnknownReadState);
    assert_eq!(error.to_string(), "chunk encode error: unknown read state");
}

#[test]
fn test_netconnection_connect_response() {
    let encoder = ChunkEncoder::default();
    let mut buf = BytesMut::new();

    NetConnection::write_connect_response(
        &encoder,
        &mut (&mut buf).writer(),
        1.0,
        "flashver",
        31.0,
        "status",
        "idk",
        "description",
        0.0,
    )
    .unwrap();

    let mut decoder = ChunkDecoder::default();

    let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
    assert_eq!(chunk.basic_header.chunk_stream_id, 0x03);
    assert_eq!(chunk.message_header.msg_type_id as u8, 0x14);
    assert_eq!(chunk.message_header.msg_stream_id, 0);

    let mut amf0_reader = Amf0Decoder::new(&chunk.payload);
    let values = amf0_reader.decode_all().unwrap();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], Amf0Value::String("_result".into())); // command name
    assert_eq!(values[1], Amf0Value::Number(1.0)); // transaction id
    assert_eq!(
        values[2],
        Amf0Value::Object(Cow::Owned(vec![
            ("fmsVer".into(), Amf0Value::String("flashver".into())),
            ("capabilities".into(), Amf0Value::Number(31.0)),
        ]))
    ); // command object
    assert_eq!(
        values[3],
        Amf0Value::Object(Cow::Owned(vec![
            ("level".into(), Amf0Value::String("idk".into())),
            ("code".into(), Amf0Value::String("status".into())),
            ("description".into(), Amf0Value::String("description".into())),
            ("objectEncoding".into(), Amf0Value::Number(0.0)),
        ]))
    ); // info object
}

#[test]
fn test_netconnection_create_stream_response() {
    let encoder = ChunkEncoder::default();
    let mut buf = BytesMut::new();

    NetConnection::write_create_stream_response(&encoder, &mut (&mut buf).writer(), 1.0, 1.0).unwrap();

    let mut decoder = ChunkDecoder::default();

    let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
    assert_eq!(chunk.basic_header.chunk_stream_id, 0x03);
    assert_eq!(chunk.message_header.msg_type_id as u8, 0x14);
    assert_eq!(chunk.message_header.msg_stream_id, 0);

    let mut amf0_reader = Amf0Decoder::new(&chunk.payload);
    let values = amf0_reader.decode_all().unwrap();

    assert_eq!(values.len(), 4);
    assert_eq!(values[0], Amf0Value::String("_result".into())); // command name
    assert_eq!(values[1], Amf0Value::Number(1.0)); // transaction id
    assert_eq!(values[2], Amf0Value::Null); // command object
    assert_eq!(values[3], Amf0Value::Number(1.0)); // stream id
}
