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
