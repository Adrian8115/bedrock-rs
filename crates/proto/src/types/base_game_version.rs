use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[repr(transparent)]
pub struct BaseGameVersion(pub String);
