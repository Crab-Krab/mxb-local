/**
 * MXB Local fork switches for the UI.
 *
 * FrostMod (shown in the app as "Game Integration") is upstream's in-game component. This fork
 * never installs it (`ALLOW_NATIVE_DOWNLOADS` in `src-tauri/src/local_security.rs`), so
 * everything that only works with it running is hidden: the consent dialog, the top-bar
 * status, its Settings section, paint sync, instant preset refresh, the protected-paints
 * prompt and the server-browser repair. The code stays so upstream changes still merge.
 *
 * Folder names on disk that contain "FrostMod" (`FrostMod Models`, `FrostMod Sounds`,
 * `FrostMod ReShade`) are kept. They are shared with upstream and with existing installs.
 */
export const GAME_INTEGRATION_SUPPORTED = false;

/**
 * The Online section: the server browser (master server, pings, the shared server list from
 * the upstream service), Ranked (mxb-ranked.com), friends, the queue for a full server and
 * joining a server from the app. MXB Local does not offer it: Online leaves the rail, the app
 * opens on Mods, and `mxb://server` links are ignored. The backend switch of the same name in
 * `src-tauri/src/local_security.rs` stops its background work and refuses its commands.
 */
export const ONLINE_SUPPORTED = false;

/**
 * Linking an account on the upstream service (api.mxbsecure.com), and what only works with
 * one: protected purchases, uploading to My mods, and the Steam link status. Settings and
 * setup no longer offer it. See `ALLOW_UPSTREAM_ACCOUNT` in `local_security.rs`.
 */
export const ACCOUNT_SUPPORTED = false;

/**
 * Share codes: uploading installed content or presets to files.catbox.moe and installing
 * what someone else's code points at. Every Share button and the paste watcher are hidden.
 * See `ALLOW_SHARE_CODES` in `local_security.rs`.
 */
export const SHARE_SUPPORTED = false;
