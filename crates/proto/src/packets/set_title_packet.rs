use bedrockrs_core::int::VAR;
use bedrockrs_proto_derive::ProtoCodec;

use crate::types::title_type::TitleType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct SetTitlePacket {
    pub title_type: TitleType,
    pub title_text: String,
    pub fade_in_time: VAR<i32>,
    pub stay_time: VAR<i32>,
    pub fade_out_time: VAR<i32>,
    pub xuid: String,
    pub platform_online_id: String,
}
