use bedrockrs_core::int::VAR;
use bedrockrs_proto_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

use super::actor_link_type::ActorLinkType;

#[derive(ProtoCodec, Debug, Clone)]
pub struct ActorLink {
    actor_unique_id_a: ActorUniqueID,
    actor_unique_id_b: ActorUniqueID,
    link_type: ActorLinkType,
    immediate: bool,
    passenger_seat_id: bool,
}
