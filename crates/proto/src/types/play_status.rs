use bedrockrs_core::int::BE;
use bedrockrs_proto_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Copy, Clone, Eq, PartialEq)]
#[enum_repr(BE::<i32>)]
pub enum PlayStatusType {
    /// Sent after Login has been successfully decoded and the player has logged in
    LoginSuccess = 0,
    /// Displays "Could not connect: Outdated client!"
    FailedClientOld = 1,
    /// Displays "Could not connect: Outdated server!"
    FailedServerOld = 2,
    /// Sent after world data to spawn the player
    PlayerSpawn = 3,
    /// Displays "Unable to connect to world. Your school does not have access to this server."
    FailedInvalidTenant = 4,
    /// Displays "The server is not running Minecraft: Education Edition. Failed to connect."
    FailedEditionMismatchEduToVanilla = 5,
    /// Displays "The server is running an incompatible edition of Minecraft. Failed to connect."
    FailedEditionMismatchVanillaToEdu = 6,
    /// Displays "Wow this server is popular! Check back later to see if space opens up. Server Full"
    FailedServerFull = 7,
    FailedEditorMismatchEditorToVanilla = 8,
    FailedEditorMismatchVanillaToEditor = 9,
}
