use io::Error as IOError;
use std::error::Error;
use std::io;
use std::sync::Arc;

use bedrockrs_proto_core::error::ProtoCodecError;
use rak_rs::connection::queue::SendQueueError;
use rak_rs::connection::RecvError;
use rak_rs::error::server::ServerError;
use thiserror::Error;

use crate::info::RAKNET_GAME_PACKET_ID;

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("Address bind error")]
    AddrBindError,
    #[error("Already Online")]
    AlreadyOnline,
    #[error("Not Listening")]
    NotListening,
    #[error("Transport Error: {0}")]
    TransportListenerError(#[from] TransportLayerError),
}

#[derive(Error, Debug, Clone)]
pub enum ConnectionError {
    #[error("IO Error: {0}")]
    IOError(#[from] Arc<IOError>),
    #[error("Proto Codec Error: {0}")]
    ProtoCodecError(#[from] ProtoCodecError),
    #[error("Connection Closed")]
    ConnectionClosed,
    #[error("Transport Error: {0}")]
    TransportError(TransportLayerError),
    #[error("Compression Error: {0}")]
    CompressError(CompressionError),
    #[error("Invalid RakNet Header, expected: {RAKNET_GAME_PACKET_ID}, got: {0}")]
    InvalidRakNetHeader(u8),
    #[error("Unknown Compression method, got: {0}")]
    UnknownCompressionMethod(u8),
    #[error("Wrong Compression method")]
    WrongCompressionMethod(u8),
}

#[derive(Error, Debug, Clone)]
pub enum CompressionError {
    #[error("Zlib Error: {0}")]
    ZlibError(#[from] Arc<dyn Error + Send + Sync>),
    #[error("Snappy Error: {0}")]
    SnappyError(#[from] Arc<IOError>),
    #[error("IO Error: {0}")]
    IOError(Arc<IOError>),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Connection Error: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Login aborted, reason: {reason}")]
    Abort { reason: String },
    #[error("Wrong protocol version (client: {client}, server: {server:?})")]
    WrongProtocolVersion { client: i32, server: Vec<i32> },
    #[error("Format Error: {0}")]
    FormatError(String),
}

#[derive(Error, Debug, Clone)]
pub enum TransportLayerError {
    #[error("IO Error: {0}")]
    IOError(#[from] Arc<IOError>),
    #[error("Raknet UDP Error: {0}")]
    RaknetUDPError(#[from] RaknetError),
    #[error("UDP Error: {0}")]
    UDPError(#[from] UdpError),
}

#[derive(Error, Debug, Clone)]
pub enum RaknetError {
    #[error("Error while Receive: {0}")]
    RecvError(#[from] RecvError),
    #[error("Error while Send: {0}")]
    SendError(SendQueueError),
    #[error("Server Error: {0}")]
    ServerError(#[from] ServerError),
    #[error("Format Error: {0}")]
    FormatError(String),
}

#[derive(Error, Debug, Clone)]
pub enum UdpError {
    #[error("Error while Receive: {0}")]
    RecvError(#[from] RecvError),
    #[error("Format Error: {0}")]
    FormatError(String),
}
