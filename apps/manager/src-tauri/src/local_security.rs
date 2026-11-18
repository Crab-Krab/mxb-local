pub const ALLOW_NATIVE_DOWNLOADS: bool = false;

/// Upstream sends the game server list it reads to the upstream control plane, so it can serve
/// that list to other installs. MXB Local does not upload anything about the user's session.
pub const ALLOW_ROSTER_CONTRIBUTIONS: bool = false;

/// The Online section: server browser, master-server sign-in, shared server list, friends,
/// the queue for a full server, joining a server from the app, and Ranked (mxb-ranked.com).
/// MXB Local leaves it out. Off, its background work never starts and its commands refuse.
pub const ALLOW_ONLINE: bool = false;

/// An account on the upstream service (api.mxbsecure.com), which upstream creates by itself
/// the first time a feature needs one. Off, none is created, a saved one is dropped at
/// startup, and the features that only work with one (protected purchases, My mods uploads,
/// paint sync, the Steam link) refuse.
pub const ALLOW_UPSTREAM_ACCOUNT: bool = false;

/// Share codes: uploading installed content or presets to files.catbox.moe, and installing
/// what a code points at. Off, those commands refuse.
pub const ALLOW_SHARE_CODES: bool = false;

/// Commands that belong to the Online section.
const ONLINE_COMMANDS: &[&str] = &[
    "friends_list", "friends_search", "friends_request", "friends_respond", "friends_remove",
    "friends_hide_presence", "set_friends_presence",
    "join_server", "join_listed_server", "close_and_join", "close_and_join_listed_server",
    "queue_join", "queue_leave", "queue_status", "queue_counts", "set_queue_restart_game",
    "list_master_servers", "set_server_browser_active", "cached_master_servers",
    "master_status", "connection_selftest", "reset_server_browser",
    "register_server_address", "probe_server",
    "saved_servers", "add_saved_server", "edit_saved_server", "remove_saved_server",
    "reorder_saved_servers", "probe_saved_servers",
    "server_riders", "servers_with_paint_sync", "supporter_servers", "cp_servers",
    "guess_server_track", "server_track_previews", "invalidate_server_track_cache",
    "server_track_inactive", "server_track_catalog",
    "ranked_identity", "ranked_profile", "set_ranked_guid",
];

/// Commands that need an upstream account.
const ACCOUNT_COMMANDS: &[&str] = &[
    "enroll_account", "set_guid", "steam_link_start", "steam_link_status",
    "set_mxbsecure_enabled", "mxbsecure_unlock", "mxbsecure_auto_unlock",
    "mxbsecure_repair_keys", "mxbsecure_status",
    "mod_upload_inspect", "mod_upload_start", "mod_upload_jobs", "mod_upload_pause",
    "mod_upload_resume", "mod_upload_cancel", "mod_upload_dismiss", "my_mods", "mod_edit",
    "mod_details", "mod_delete", "mod_set_thumb",
    "publish_paints", "sync_paints", "paint_policies", "set_paint_policy",
    "supporter_refresh", "supporter_email_send", "supporter_email_confirm",
    "supporter_checkout",
];

/// Commands that upload to or download from a share host.
const SHARE_COMMANDS: &[&str] = &[
    "preset_bundle_create", "preset_bundle_import",
    "file_share_plan", "file_share_create", "file_share_preview", "file_share_import",
    "live_share_publish", "live_share_preview", "live_share_subscribe", "live_share_sync",
    "live_share_check", "live_share_list", "live_share_forget", "live_share_set_auto",
    "live_share_owner_code", "live_share_adopt", "share_logs",
];

/// Why `command` is refused in this build, or `None` when it may run. Checked for every
/// command before it reaches its handler (`ipc_allowed` in `main.rs`), so a hidden button
/// that is somehow still reachable can't start a feature this fork leaves out.
pub fn refused_command(command: &str) -> Option<&'static str> {
    if !ALLOW_ONLINE && ONLINE_COMMANDS.contains(&command) {
        return Some("Online features are not available in MXB Local");
    }
    if !ALLOW_UPSTREAM_ACCOUNT && ACCOUNT_COMMANDS.contains(&command) {
        return Some("Upstream accounts are not available in MXB Local");
    }
    if !ALLOW_SHARE_CODES && SHARE_COMMANDS.contains(&command) {
        return Some("Share codes are not available in MXB Local");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_account_and_share_commands_are_refused() {
        for c in ["list_master_servers", "ranked_profile", "friends_list", "queue_join", "join_server"] {
            assert!(refused_command(c).is_some(), "{c}");
        }
        for c in ["enroll_account", "my_mods", "mxbsecure_unlock", "steam_link_start"] {
            assert!(refused_command(c).is_some(), "{c}");
        }
        for c in ["file_share_create", "live_share_publish", "preset_bundle_import"] {
            assert!(refused_command(c).is_some(), "{c}");
        }
    }

    #[test]
    fn everyday_commands_still_run() {
        for c in [
            "launch_game", "search_mods", "get_mod_ratings", "get_installed_mods",
            "apply_model_swap", "presets_apply", "feel_apply", "mods_state_restore_all",
            "presets_export", "get_config",
        ] {
            assert_eq!(refused_command(c), None, "{c}");
        }
    }
}
