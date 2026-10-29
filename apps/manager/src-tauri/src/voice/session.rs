//! Server-name normalization shared with paint sync and friend presence.
//! No voice session or background worker is built in MXB Local.
pub fn room_key(server_name: &str) -> String {
    server_name.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}
