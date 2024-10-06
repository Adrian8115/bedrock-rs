use super::item_stack_id_variant::ItemStackIdVariant;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum ItemStackDescriptor {
    Invalid,
    Valid {
        id: VAR<i32>,
        stack_size: LE<u16>,
        aux_value: LE<u16>,
        include_net_id: bool,
        include_net_id_data: Option<ItemStackIdVariant>,
        block_runtime_id: VAR<i32>,
        user_data_buffer: String,
    },
}
impl ProtoCodec for ItemStackDescriptor {
    fn proto_serialize(
        &self,
        stream: &mut Vec<u8>,
    ) -> Result<(), bedrockrs_proto_core::error::ProtoCodecError> {
        match self {
            ItemStackDescriptor::Invalid { id } => id.proto_serialize(stream)?,
            ItemStackDescriptor::Valid {
                id,
                stack_size,
                aux_value,
                include_net_id,
                include_net_id_data,
                block_runtime_id,
                user_data_buffer,
            } => {
                id.proto_serialize(stream)?;
                stack_size.proto_serialize(stream)?;
                aux_value.proto_serialize(stream)?;
                include_net_id.proto_serialize(stream)?;
                include_net_id_data.proto_serialize(stream)?;
                block_runtime_id.proto_serialize(stream)?;
                user_data_buffer.proto_serialize(stream)?;
            }
        };

        Ok(())
    }

    fn proto_deserialize(
        stream: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, bedrockrs_proto_core::error::ProtoCodecError> {
        let id = VAR::<i32>::proto_deserialize(stream)?;
        if id.into_inner() == 0 {
            Ok(Self::Invalid { id })
        } else {
            let stack_size = LE::<u16>::proto_deserialize(stream)?;
            let aux_value = LE::<u16>::proto_deserialize(stream)?;
            let include_net_id = bool::proto_deserialize(stream)?;

            let include_net_id_data = if include_net_id {
                Some(ItemStackIdVariant::proto_deserialize(stream)?)
            } else {
                None
            };

            Ok(Self::Valid {
                id,
                stack_size,
                aux_value,
                include_net_id,
                include_net_id_data,
                block_runtime_id: VAR::<i32>::proto_deserialize(stream)?,
                user_data_buffer: String::proto_deserialize(stream)?,
            })
        }
    }
}