use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ToastRequestPacket {
    pub title: String,
    pub content: String,
}
