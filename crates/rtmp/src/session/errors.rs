use std::fmt;

use crate::channels::UniqueID;
use crate::chunk::ChunkDecodeError;
use crate::handshake::HandshakeError;
use crate::macros::from_error;
use crate::messages::MessageError;
use crate::netconnection::NetConnectionError;
use crate::netstream::NetStreamError;
use crate::protocol_control_messages::ProtocolControlMessageError;
use crate::user_control_messages::EventMessagesError;

#[derive(Debug)]
pub enum SessionError {
    Handshake(HandshakeError),
    Message(MessageError),
    ChunkDecode(ChunkDecodeError),
    ProtocolControlMessage(ProtocolControlMessageError),
    NetStream(NetStreamError),
    NetConnection(NetConnectionError),
    EventMessages(EventMessagesError),
    UnknownStreamID(u32),
    PublisherDisconnected(UniqueID),
    Io(std::io::Error),
    Timeout(tokio::time::error::Elapsed),
    NoAppName,
    NoStreamName,
    PublishRequestDenied,
    ConnectRequestDenied,
    PlayNotSupported,
    PublisherDropped,
    InvalidChunkSize(usize),
}

impl SessionError {
    pub fn is_client_closed(&self) -> bool {
        match self {
            Self::Io(err) => matches!(
                err.kind(),
                std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::UnexpectedEof
            ),
            Self::Timeout(_) => true,
            _ => false,
        }
    }
}

from_error!(SessionError, Self::Handshake, HandshakeError);
from_error!(SessionError, Self::Message, MessageError);
from_error!(SessionError, Self::ChunkDecode, ChunkDecodeError);
from_error!(SessionError, Self::ProtocolControlMessage, ProtocolControlMessageError);
from_error!(SessionError, Self::NetStream, NetStreamError);
from_error!(SessionError, Self::NetConnection, NetConnectionError);
from_error!(SessionError, Self::EventMessages, EventMessagesError);
from_error!(SessionError, Self::Io, std::io::Error);
from_error!(SessionError, Self::Timeout, tokio::time::error::Elapsed);

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "io error: {}", error),
            Self::Handshake(error) => write!(f, "handshake error: {}", error),
            Self::Message(error) => write!(f, "message error: {}", error),
            Self::ChunkDecode(error) => write!(f, "chunk decode error: {}", error),
            Self::ProtocolControlMessage(error) => {
                write!(f, "protocol control message error: {}", error)
            }
            Self::NetStream(error) => write!(f, "netstream error: {}", error),
            Self::NetConnection(error) => write!(f, "netconnection error: {}", error),
            Self::EventMessages(error) => write!(f, "event messages error: {}", error),
            Self::UnknownStreamID(id) => write!(f, "unknown stream id: {}", id),
            Self::PublisherDisconnected(name) => write!(f, "publisher disconnected: {}", name),
            Self::NoAppName => write!(f, "no app name"),
            Self::NoStreamName => write!(f, "no stream name"),
            Self::PublishRequestDenied => write!(f, "publish request denied"),
            Self::ConnectRequestDenied => write!(f, "connect request denied"),
            Self::InvalidChunkSize(size) => write!(f, "invalid chunk size: {}", size),
            Self::PlayNotSupported => write!(f, "play not supported"),
            Self::PublisherDropped => write!(f, "publisher dropped"),
            Self::Timeout(error) => write!(f, "timeout: {}", error),
        }
    }
}
