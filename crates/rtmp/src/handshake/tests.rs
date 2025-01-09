use std::io::{Cursor, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::Bytes;

use super::{HandshakeError, HandshakeServer};
use crate::handshake::define::{
    SchemaVersion, {self},
};
use crate::handshake::digest::DigestProcessor;
use crate::handshake::errors::DigestError;
use crate::handshake::ServerHandshakeState;

#[test]
fn test_simple_handshake() {
    let mut handshake_server = HandshakeServer::default();

    let mut c0c1 = Vec::with_capacity(1528 + 8);
    c0c1.write_u8(3).unwrap(); // version
    c0c1.write_u32::<BigEndian>(123).unwrap(); // timestamp
    c0c1.write_u32::<BigEndian>(0).unwrap(); // zero

    for i in 0..1528 {
        c0c1.write_u8((i % 256) as u8).unwrap();
    }

    let c0c1 = Bytes::from(c0c1);

    let mut writer = Vec::new();
    handshake_server
        .handshake(&mut std::io::Cursor::new(c0c1.clone()), &mut writer)
        .unwrap();

    let mut reader = Cursor::new(writer);
    assert_eq!(reader.read_u8().unwrap(), 3); // version
    let timestamp = reader.read_u32::<BigEndian>().unwrap(); // timestamp
    assert_eq!(reader.read_u32::<BigEndian>().unwrap(), 0); // zero

    let mut server_random = vec![0; 1528];
    reader.read_exact(&mut server_random).unwrap();

    assert_eq!(reader.read_u32::<BigEndian>().unwrap(), 123); // our timestamp
    let timestamp2 = reader.read_u32::<BigEndian>().unwrap(); // server timestamp

    assert!(timestamp2 >= timestamp);

    let mut read_client_random = vec![0; 1528];
    reader.read_exact(&mut read_client_random).unwrap();

    assert_eq!(&c0c1[9..], &read_client_random);

    let mut c2 = Vec::with_capacity(1528 + 8);
    c2.write_u32::<BigEndian>(timestamp).unwrap(); // timestamp
    c2.write_u32::<BigEndian>(124).unwrap(); // our timestamp
    c2.write_all(&server_random).unwrap();

    let mut writer = Vec::new();
    handshake_server
        .handshake(&mut std::io::Cursor::new(Bytes::from(c2)), &mut writer)
        .unwrap();

    assert_eq!(handshake_server.state(), ServerHandshakeState::Finish)
}

#[test]
fn test_complex_handshake() {
    let mut handshake_server = HandshakeServer::default();

    let mut writer = Vec::with_capacity(3073);
    writer.write_u8(3).unwrap(); // version

    let mut c0c1 = Vec::with_capacity(1528 + 8);
    c0c1.write_u32::<BigEndian>(123).unwrap(); // timestamp
    c0c1.write_u32::<BigEndian>(100).unwrap(); // client version

    for i in 0..1528 {
        c0c1.write_u8((i % 256) as u8).unwrap();
    }

    let data_digest = DigestProcessor::new(Bytes::from(c0c1), define::RTMP_CLIENT_KEY_FIRST_HALF);

    let (first, second, third) = data_digest.generate_and_fill_digest(SchemaVersion::Schema1).unwrap();

    writer.extend_from_slice(&first);
    writer.extend_from_slice(&second);
    writer.extend_from_slice(&third);

    let mut bytes = Vec::new();
    handshake_server
        .handshake(&mut std::io::Cursor::new(Bytes::from(writer)), &mut bytes)
        .unwrap();

    let s0 = &bytes[0..1];
    let s1 = &bytes[1..1537];
    let s2 = &bytes[1537..3073];

    assert_eq!(s0[0], 3); // version
    assert_ne!((&s1[..4]).read_u32::<BigEndian>().unwrap(), 0); // timestamp should not be zero
    assert_eq!((&s1[4..8]).read_u32::<BigEndian>().unwrap(), define::RTMP_SERVER_VERSION); // RTMP version

    let data_digest = DigestProcessor::new(Bytes::copy_from_slice(s1), define::RTMP_SERVER_KEY_FIRST_HALF);

    let (digest, schema) = data_digest.read_digest().unwrap();
    assert_eq!(schema, SchemaVersion::Schema1);

    assert_ne!((&s2[..4]).read_u32::<BigEndian>().unwrap(), 0); // timestamp should not be zero
    assert_eq!((&s2[4..8]).read_u32::<BigEndian>().unwrap(), 123); // our timestamp

    let key_digest = DigestProcessor::new(Bytes::new(), define::RTMP_SERVER_KEY);

    let key = key_digest.make_digest(&second, &[]).unwrap();
    let data_digest = DigestProcessor::new(Bytes::new(), &key);

    assert_eq!(data_digest.make_digest(&s2[..1504], &[]).unwrap(), s2[1504..]);

    let key = key_digest.make_digest(&digest, &[]).unwrap();
    let data_digest = DigestProcessor::new(Bytes::new(), &key);

    let mut c2 = Vec::new();
    for i in 0..1528 {
        c2.write_u8((i % 256) as u8).unwrap();
    }

    let digest = data_digest.make_digest(&c2, &[]).unwrap();

    let mut c2 = Vec::with_capacity(1528 + 8);
    c2.write_u32::<BigEndian>(123).unwrap(); // timestamp
    c2.write_u32::<BigEndian>(124).unwrap(); // our timestamp
    c2.write_all(&digest).unwrap();

    let mut writer = Vec::new();
    handshake_server
        .handshake(&mut std::io::Cursor::new(Bytes::from(c2)), &mut writer)
        .unwrap();

    assert_eq!(handshake_server.state(), ServerHandshakeState::Finish)
}

#[test]
fn test_error_display() {
    let err = HandshakeError::Digest(DigestError::CannotGenerate);
    assert_eq!(err.to_string(), "digest error: cannot generate digest");

    let err = HandshakeError::Digest(DigestError::DigestLengthNotCorrect);
    assert_eq!(err.to_string(), "digest error: digest length not correct");

    let err = HandshakeError::Digest(DigestError::UnknownSchema);
    assert_eq!(err.to_string(), "digest error: unknown schema");

    let err = HandshakeError::Digest(DigestError::NotEnoughData);
    assert_eq!(err.to_string(), "digest error: not enough data");

    let err = HandshakeError::IO(Cursor::new(Vec::<u8>::new()).read_u8().unwrap_err());
    // no idea why this io error is the error we get but this is mainly testing the
    // display impl anyway
    assert_eq!(err.to_string(), "io error: failed to fill whole buffer");
}
