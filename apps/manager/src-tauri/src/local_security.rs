pub const ALLOW_NATIVE_DOWNLOADS: bool = false;

/// Upstream sends the game server list it reads to the upstream control plane, so it can serve
/// that list to other installs. MXB Local does not upload anything about the user's session.
pub const ALLOW_ROSTER_CONTRIBUTIONS: bool = false;
