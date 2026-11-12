//! Windows user-bound secret storage for this independent local fork.
//! Encryption failure is an error: credentials must never fall back to plaintext.
use anyhow::{bail, Result};
use base64::{engine::general_purpose::STANDARD, Engine};

const PREFIX: &str = "dpapi:v1:";

pub fn seal(plain: &[u8]) -> Result<String> {
    Ok(format!("{PREFIX}{}", STANDARD.encode(crypt(plain, false)?)))
}

pub fn open(stored: &str) -> Result<Vec<u8>> {
    let encoded = stored.strip_prefix(PREFIX)
        .ok_or_else(|| anyhow::anyhow!("Refusing an unencrypted credential file"))?;
    crypt(&STANDARD.decode(encoded)?, true)
}

#[cfg(windows)]
fn crypt(input: &[u8], decrypt: bool) -> Result<Vec<u8>> {
    use std::{ffi::c_void, ptr};
    #[repr(C)]
    struct Blob { len: u32, data: *mut u8 }
    #[link(name = "Crypt32")]
    extern "system" {
        fn CryptProtectData(input: *const Blob, description: *const u16, entropy: *const Blob,
            reserved: *mut c_void, prompt: *mut c_void, flags: u32, output: *mut Blob) -> i32;
        fn CryptUnprotectData(input: *const Blob, description: *mut *mut u16, entropy: *const Blob,
            reserved: *mut c_void, prompt: *mut c_void, flags: u32, output: *mut Blob) -> i32;
    }
    #[link(name = "Kernel32")]
    extern "system" { fn LocalFree(memory: *mut c_void) -> *mut c_void; }
    let input_len = u32::try_from(input.len())?;
    let source = Blob { len: input_len, data: input.as_ptr() as *mut u8 };
    let entropy = b"MXB Local credential storage v1";
    let entropy = Blob { len: entropy.len() as u32, data: entropy.as_ptr() as *mut u8 };
    let mut output = Blob { len: 0, data: ptr::null_mut() };
    // No LOCAL_MACHINE flag: only this Windows user can decrypt the result.
    // CRYPTPROTECT_UI_FORBIDDEN prevents credential prompts from hanging a background save.
    let result = unsafe {
        if decrypt {
            CryptUnprotectData(&source, ptr::null_mut(), &entropy, ptr::null_mut(), ptr::null_mut(), 1, &mut output)
        } else {
            CryptProtectData(&source, ptr::null(), &entropy, ptr::null_mut(), ptr::null_mut(), 1, &mut output)
        }
    };
    if result == 0 { return Err(std::io::Error::last_os_error().into()); }
    if output.data.is_null() { bail!("Windows returned no protected credential data"); }
    let bytes = unsafe { std::slice::from_raw_parts(output.data, output.len as usize).to_vec() };
    unsafe { LocalFree(output.data.cast()); }
    Ok(bytes)
}

#[cfg(not(windows))]
fn crypt(_input: &[u8], _decrypt: bool) -> Result<Vec<u8>> {
    bail!("This fork's credential storage requires Windows")
}

fn visit_config(doc: &mut serde_json::Value, decrypt: bool) -> Result<()> {
    fn field(value: Option<&mut serde_json::Value>, decrypt: bool, name: &str) -> Result<()> {
        if let Some(value) = value {
            if let Some(text) = value.as_str().filter(|s| !s.is_empty()) {
                let result = if decrypt { open_or_sign_out(text, name) } else { seal(text.as_bytes())? };
                *value = serde_json::Value::String(result);
            }
        }
        Ok(())
    }
    field(doc.get_mut("cpToken"), decrypt, "account token")?;
    if let Some(servers) = doc.get_mut("servers").and_then(|v| v.as_array_mut()) {
        for server in servers { field(server.get_mut("token"), decrypt, "server token")?; }
    }
    Ok(())
}

/// A saved credential that can't be opened is treated as signed out, not as a broken file.
/// Windows can no longer decrypt DPAPI data after an administrator resets the password, after
/// the profile is recreated, or once the file is copied to another user or PC. Failing the
/// whole config there made the app rebuild default settings and lose every other preference.
/// Encryption is still strict: `seal` failing is an error and nothing is written in plaintext.
fn open_or_sign_out(stored: &str, name: &str) -> String {
    match open(stored).and_then(|bytes| Ok(String::from_utf8(bytes)?)) {
        Ok(plain) => plain,
        Err(e) => {
            log::warn!("saved {name} could not be opened ({e:#}), treating it as signed out");
            String::new()
        }
    }
}

pub fn seal_config(doc: &mut serde_json::Value) -> Result<()> { visit_config(doc, false) }
pub fn open_config(doc: &mut serde_json::Value) -> Result<()> { visit_config(doc, true) }

#[cfg(test)]
mod portable_tests {
    use super::*;
    #[test]
    fn an_unreadable_credential_signs_out_and_keeps_everything_else() {
        // Plaintext is refused on every platform, so it stands in for a token DPAPI can't open.
        let mut doc = serde_json::json!({
            "cpToken": "copied-from-another-app",
            "servers": [{"token": "dpapi:v1:bm90LXJlYWwtZGF0YQ==", "url": "https://host"}],
            "modsPath": "C:/Games",
        });
        open_config(&mut doc).unwrap();
        assert_eq!(doc["cpToken"], "");
        assert_eq!(doc["servers"][0]["token"], "");
        assert_eq!(doc["servers"][0]["url"], "https://host");
        assert_eq!(doc["modsPath"], "C:/Games");
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn credentials_round_trip_without_plaintext_at_rest() {
        let mut doc = serde_json::json!({"cpToken":"private-account-token", "servers":[{"token":"private-server-token"}], "modsPath":"C:/Games"});
        let original = doc.clone();
        seal_config(&mut doc).unwrap();
        let disk = doc.to_string();
        assert!(!disk.contains("private-account-token"));
        assert!(!disk.contains("private-server-token"));
        assert!(disk.contains("C:/Games"));
        open_config(&mut doc).unwrap();
        assert_eq!(doc, original);
    }
    #[test]
    fn tampering_and_plaintext_are_rejected() {
        assert!(open("plaintext-cookie").is_err());
        let encrypted = seal(b"cookie-value").unwrap();
        assert_eq!(open(&encrypted).unwrap(), b"cookie-value");
        let mut bytes = STANDARD.decode(encrypted.strip_prefix(PREFIX).unwrap()).unwrap();
        let last = bytes.len()-1;
        bytes[last] ^= 1;
        assert!(open(&format!("{PREFIX}{}", STANDARD.encode(bytes))).is_err());
    }
}
