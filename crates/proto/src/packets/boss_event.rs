use bedrockrs_proto_derive::{gamepacket, ProtoCodec};
use bedrockrs_shared::actor_unique_id::ActorUniqueID;
use crate::types::event_type::BossEventType;

#[gamepacket(id = 74)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct BossEventPacket {
    actor_id: ActorUniqueID,
    event_type: BossEventType,
}