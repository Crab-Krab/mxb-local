//! Account helpers shared by optional paint-sync and server features.
//! Voice signalling, room sockets and peer messages are excluded from MXB Local.
use std::time::Duration;
use serde::Deserialize;
use crate::config::AppConfig;
use crate::paintsync::control_plane;
const HTTP_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Deserialize)]
struct ClaimedAccount {
    token: String,
    #[serde(rename = "riderName")]
    rider_name: String,
}

/// The token this app talks to the control plane with, claiming one if it has none.
///
/// An enrolled player already has a token and keeps it — same account, same paints. Everyone
/// else gets a self-serve one, silently, the first time voice or paint sync needs one. That
/// is the difference between "install the app and ride" and "install the app, then go and
/// ask someone for an invite code", and it is the whole reason the endpoint exists.
///
/// Returns the token and whether it is new, so the caller can save the config exactly once.
pub async fn ensure_account(cfg: &AppConfig) -> Result<(String, bool), String> {
    let existing = cfg.cp_token.trim();
    if !existing.is_empty() {
        return Ok((existing.to_string(), false));
    }

    let rider_name = rider_name(cfg);
    let client = reqwest::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .build()
        .map_err(|e| format!("Couldn't reach MXB App's service: {e}"))?;
    // The machine's one-way hash, like the shared mint in `mxb_core::account` (`device.rs`).
    let resp = mxb_core::device::with_device(
        client
            .post(format!("{}/v1/account", control_plane()))
            .json(&serde_json::json!({ "riderName": rider_name })),
    )
    .await
    .send()
    .await
    .map_err(|e| format!("Couldn't reach MXB App's service: {e}"))?;

    if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return Err("Too many new accounts from this connection today. Try again tomorrow.".into());
    }
    if !resp.status().is_success() {
        return Err(format!("MXB App's service turned down the sign-up ({}).", resp.status()));
    }
    let claimed: ClaimedAccount = resp
        .json()
        .await
        .map_err(|e| format!("MXB App's service sent something unexpected: {e}"))?;
    log::info!("[cp] claimed an account as {}", claimed.rider_name);
    Ok((claimed.token, true))
}

/// The token, claimed and saved, ready to use.
///
/// [`ensure_account`] hands back whether the account is new because only the caller can
/// write the config; every caller then does the same thing with the answer, so it lives
/// here once. Voice and paint sync share one account — they are the same rider on the same
/// server, and a second identity would put their name on the grid twice.
pub async fn account(app: &tauri::AppHandle, cfg: &AppConfig) -> Result<String, String> {
    let (token, is_new) = ensure_account(cfg).await?;
    if is_new {
        // Re-read before writing: `save` rewrites the whole file and this ran across an
        // await, so the config on disk may have moved on.
        let mut cfg = crate::config::load_or_detect(app).unwrap_or_default();
        cfg.cp_token = token.clone();
        if cfg.cp_rider_name.trim().is_empty() {
            cfg.cp_rider_name = rider_name(&cfg);
        }
        if let Err(e) = crate::config::save(app, &cfg) {
            log::warn!("[cp] couldn't save the claimed account: {e:#}");
        }
    }
    Ok(token)
}

/// The best guess at what this player is called in game.
///
/// A label, not an identity — it is what other riders see beside a talking indicator. The
/// enrolled name first, then the profile folder the game keeps (which *is* the rider name),
/// then a placeholder, because not knowing must never be a reason voice won't start.
pub fn rider_name(cfg: &AppConfig) -> String {
    let enrolled = cfg.cp_rider_name.trim();
    if !enrolled.is_empty() {
        return enrolled.to_string();
    }
    if let Some(profile) = crate::presets::list_profiles(&cfg.profiles_dir()).into_iter().next() {
        if !profile.trim().is_empty() {
            return profile;
        }
    }
    "Rider".to_string()
}
