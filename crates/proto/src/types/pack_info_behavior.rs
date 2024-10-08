use bedrockrs_core::int::LE;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub struct BehaviorPackInfoType {
    id: String,
    version: String,
    size: LE<u64>,
    content_key: String,
    sub_pack_name: String,
    content_identify: String,
    has_scripts: bool,
}
