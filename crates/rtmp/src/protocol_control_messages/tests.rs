use bytes::{BufMut, BytesMut};

use crate::chunk::{ChunkDecoder, ChunkEncodeError, ChunkEncoder};
use crate::protocol_control_messages::{
    ProtocolControlMessageError, ProtocolControlMessageReader, ProtocolControlMessagesWriter,
};

#[test]
fn test_error_display() {
    let error = ProtocolControlMessageError::ChunkEncode(ChunkEncodeError::UnknownReadState);
    assert_eq!(error.to_string(), "chunk encode error: unknown read state");

    let error = ProtocolControlMessageError::IO(std::io::Error::from(std::io::ErrorKind::Other));
    assert_eq!(error.to_string(), "io error: other error");
}

#[test]
fn test_reader_read_set_chunk_size() {
    let data = vec![0x00, 0x00, 0x00, 0x01];
    let chunk_size = ProtocolControlMessageReader::read_set_chunk_size(data.into()).unwrap();
    assert_eq!(chunk_size, 1);
}

#[test]
fn test_writer_write_set_chunk_size() {
    let encoder = ChunkEncoder::default();
    let mut buf = BytesMut::new();

    ProtocolControlMessagesWriter::write_set_chunk_size(&encoder, &mut (&mut buf).writer(), 1).unwrap();

    let mut decoder = ChunkDecoder::default();

    let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
    assert_eq!(chunk.basic_header.chunk_stream_id, 0x02);
    assert_eq!(chunk.message_header.msg_type_id as u8, 0x01);
    assert_eq!(chunk.message_header.msg_stream_id, 0);
    assert_eq!(chunk.payload, vec![0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn test_writer_window_acknowledgement_size() {
    let encoder = ChunkEncoder::default();
    let mut buf = BytesMut::new();

    ProtocolControlMessagesWriter::write_window_acknowledgement_size(&encoder, &mut (&mut buf).writer(), 1).unwrap();

    let mut decoder = ChunkDecoder::default();

    let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
    assert_eq!(chunk.basic_header.chunk_stream_id, 0x02);
    assert_eq!(chunk.message_header.msg_type_id as u8, 0x05);
    assert_eq!(chunk.message_header.msg_stream_id, 0);
    assert_eq!(chunk.payload, vec![0x00, 0x00, 0x00, 0x01]);
}

#[test]
fn test_writer_set_peer_bandwidth() {
    let encoder = ChunkEncoder::default();
    let mut buf = BytesMut::new();

    ProtocolControlMessagesWriter::write_set_peer_bandwidth(&encoder, &mut (&mut buf).writer(), 1, 2).unwrap();

    let mut decoder = ChunkDecoder::default();

    let chunk = decoder.read_chunk(&mut buf).expect("read chunk").expect("chunk");
    assert_eq!(chunk.basic_header.chunk_stream_id, 0x02);
    assert_eq!(chunk.message_header.msg_type_id as u8, 0x06);
    assert_eq!(chunk.message_header.msg_stream_id, 0);
    assert_eq!(chunk.payload, vec![0x00, 0x00, 0x00, 0x01, 0x02]);
}
