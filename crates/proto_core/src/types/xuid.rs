use std::io::Cursor;
use xuid::Xuid;
use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl ProtoCodec for Xuid {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        self.to_string().proto_serialize(buf)?;
        Ok(())
    }

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Xuid::try_from(String::proto_deserialize(buf)?).map_err(ProtoCodecError::XuidParseError)
    }
}