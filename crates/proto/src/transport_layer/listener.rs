use crate::error::{RaknetError, TransportLayerError};
use crate::transport_layer::TransportLayerConnection;

pub enum TransportLayerListener {
    RaknetUDP(rak_rs::Listener),
    NetherNet(/* TODO */),
}

impl TransportLayerListener {
    pub async fn start(&mut self) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerListener::RaknetUDP(listener) => match listener.start().await {
                Ok(_) => Ok(()),
                Err(e) => Err(TransportLayerError::RaknetUDPError(
                    RaknetError::ServerError(e),
                )),
            },
            _ => {
                todo!()
            }
        }
    }

    pub async fn accept(&mut self) -> Result<TransportLayerConnection, TransportLayerError> {
        match self {
            TransportLayerListener::RaknetUDP(listener) => match listener.accept().await {
                Ok(conn) => Ok(TransportLayerConnection::RaknetUDP(conn)),
                Err(e) => Err(TransportLayerError::RaknetUDPError(
                    RaknetError::ServerError(e),
                )),
            },
            _ => {
                todo!()
            }
        }
    }
}
