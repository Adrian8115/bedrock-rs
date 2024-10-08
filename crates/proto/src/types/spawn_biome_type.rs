use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::int::LE;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(LE::<i16>)]
pub enum SpawnBiomeType {
    Default = 0,
    UserDefined = 1,
}
