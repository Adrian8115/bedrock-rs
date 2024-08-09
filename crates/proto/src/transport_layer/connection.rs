use std::io::{Cursor, Write};
use std::sync::Arc;

use bedrockrs_core::int::LE;

use crate::error::{RaknetError, TransportLayerError, UdpError};
use crate::info::RAKNET_GAME_PACKET_ID;

pub enum TransportLayerConnection {
    RaknetUDP(rak_rs::connection::Connection),
    // TODO RaknetTCP(...),
    NetherNet(/* TODO */),
    // TODO Quic(s2n_quic::connection::Connection),
    // Tcp(std::net::TcpStream),
    Udp(tokio::net::UdpSocket) //MCBE over UDP :sobb:??
}

impl TransportLayerConnection {
    pub async fn send(&mut self, stream: &Cursor<&[u8]>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut final_stream = vec![];

                LE::<u8>::write(&LE::new(RAKNET_GAME_PACKET_ID), &mut final_stream)
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;

                final_stream
                    .write_all(stream.get_ref())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;

                // TODO Find out if immediate: true should be used
                conn.send(final_stream.as_slice(), true)
                    .await
                    .map_err(|e| TransportLayerError::RaknetUDPError(RaknetError::SendError(e)))
            }
            TransportLayerConnection::Udp(conn) => {
                //BIO 
                let mut final_stream = vec![];

                LE::<u8>::write(&LE::new(RAKNET_GAME_PACKET_ID), &mut final_stream)
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;

                final_stream
                    .write_all(stream.get_ref())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;
                
                conn.send(final_stream.as_slice()) 
                    .await
                    .map(|_| ())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e))) //udp send error is std::io::Error
            }
            _ => {
                todo!()
            }
        }
    }

    pub async fn recv(&mut self, stream: &mut Vec<u8>) -> Result<(), TransportLayerError> {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                let mut recv_stream = conn
                    .recv()
                    .await
                    .map_err(|e| TransportLayerError::RaknetUDPError(RaknetError::RecvError(e)))?;

                let mut recv_stream = Cursor::new(recv_stream.as_slice());

                match LE::<u8>::read(&mut recv_stream)
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?
                    .into_inner()
                {
                    RAKNET_GAME_PACKET_ID => {}
                    other => {
                        return Err(TransportLayerError::RaknetUDPError(
                            RaknetError::FormatError(format!(
                                "Expected Raknet Game Packet ID ({:?}), got: {:?}",
                                RAKNET_GAME_PACKET_ID, other
                            )),
                        ));
                    }
                };

                Ok(stream
                    .write_all(recv_stream.into_inner())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?)
            }

            TransportLayerConnection::Udp(conn) => {
                // TODO: Configure the socket for non-blocking (NIO) mode to improve efficiency and responsiveness.
                // Non-blocking I/O allows the program to perform other tasks while waiting for I/O operations to complete,
                // thereby avoiding potential bottlenecks and enhancing overall performance in high-concurrency scenarios.
                let mut recv_buffer: Vec<u8> = vec![0; 4096]; 
                let _amt = conn
                    .recv(&mut recv_buffer)
                    .await
                    .map(|_| ())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?;
                
                let mut recv_buffer = Cursor::new(recv_buffer.as_slice());

                match LE::<u8>::read(&mut recv_buffer)
                .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?
                .into_inner()
                {
                    RAKNET_GAME_PACKET_ID => {}
                    other => {
                        return Err(TransportLayerError::UDPError(
                            UdpError::FormatError(format!(
                                "Expected RakNet Game Packet ID ({:?}), got: {:?}",
                                RAKNET_GAME_PACKET_ID, other
                            )),
                        ));
                    }
                };

                Ok(stream
                    .write_all(recv_buffer.into_inner())
                    .map_err(|e| TransportLayerError::IOError(Arc::new(e)))?)
            }

            _ => {
                todo!()
            }
        }
    }

    pub async fn close(self) {
        match self {
            TransportLayerConnection::RaknetUDP(conn) => {
                conn.close().await;
            }
            TransportLayerConnection::Udp(socket) => {
                std::mem::drop(socket); //for closing the socket explicitly(...do we need it?)
            }
            _ => {
                todo!()
            }
        }
    }
}
