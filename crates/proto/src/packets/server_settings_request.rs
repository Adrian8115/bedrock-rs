use bedrockrs_proto_derive::{gamepacket, ProtoCodec};

#[gamepacket(id = 102)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct ServerSettingsRequestPacket {}
