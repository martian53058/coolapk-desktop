//! iOS-compatible Coolapk API identity used by the supplied working mobile client.
//!
//! Keep these fields aligned with `IosHeaders.kt` in the supplied c001apk_next source.
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use md5::{Digest, Md5};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::RequestBuilder;
use std::time::{SystemTime, UNIX_EPOCH};

pub const APP_ID: &str = "com.coolapk.app";
pub const APP_VERSION: &str = "16.6.1";
pub const APP_CODE: &str = "2610151";
pub const API_VERSION: &str = "16";
pub const SDK_LOCALE: &str = "zh-HK";
pub const ACCEPT_LANGUAGE: &str = "zh-Hans-HK;q=1.0, zh-Hant-HK;q=0.9, en-HK;q=0.8";
const TOKEN_PREFIX: &str = "token://com.coolapk.app/1e7870f7312eb435aef6b59b211a9880?";

// One coherent profile matching a profile used by the reference app. The mobile client
// persists a profile; this desktop port intentionally starts with a fixed valid profile.
pub const MODEL: &str = "iPhone 16";
pub const HARDWARE_ID: &str = "iPhone17,3";
pub const SALES_MODEL: &str = "MYEV3CH/A";
pub const SYSTEM_VERSION: &str = "26.7";

pub fn default_user_agent() -> String {
    format!(
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 (#Build; Apple; {HARDWARE_ID}; {MODEL}; iOS_{SYSTEM_VERSION}) +iCoolMarket/{APP_VERSION}-{APP_CODE}"
    )
}

pub fn app_device(device_id: &str) -> String {
    app_device_for_profile(device_id, MODEL, HARDWARE_ID, SALES_MODEL)
}

pub fn app_device_for_profile(device_id: &str, model: &str, hardware_id: &str, sales_model: &str) -> String {
    let identity = if sales_model.is_empty() {
        hardware_id.to_string()
    } else {
        format!("{hardware_id}/{sales_model}")
    };
    let plain = format!("{device_id}; ; ; ; Apple; Apple; {model}; {identity}");
    BASE64.encode(plain.as_bytes()).chars().rev().collect::<String>()
        .trim_start_matches('=').to_string()
}

pub fn app_token(device_id: &str, timestamp_seconds: u64) -> String {
    let time_md5 = md5_hex(timestamp_seconds.to_string().as_bytes());
    let material = format!("{TOKEN_PREFIX}{time_md5}${device_id}&{APP_ID}");
    let digest = md5_hex(BASE64.encode(material.as_bytes()).as_bytes());
    format!("{digest}{device_id}0x{:x}", timestamp_seconds as u32)
}

pub fn current_timestamp_seconds() -> Result<u64, String> {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| format!("system clock error: {error}"))
}

pub fn apply_headers(request: RequestBuilder, device_id: &str) -> Result<RequestBuilder, String> {
    let timestamp = current_timestamp_seconds()?;

    // reqwest::RequestBuilder::header appends values. The client has default
    // profile headers, and several legacy call sites also add X-App-Token /
    // X-Requested-With before reaching this function. Build one HeaderMap and
    // merge it with replacement semantics so the wire request has exactly one
    // coherent iOS profile (especially one token generated for this device ID).
    let user_agent = default_user_agent();
    let device = app_device(device_id);
    let token = app_token(device_id, timestamp);
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", HeaderValue::from_static(ACCEPT_LANGUAGE));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&user_agent).map_err(|_| "invalid iOS User-Agent".to_string())?,
    );
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
    headers.insert("X-Sdk-Int", HeaderValue::from_static(SYSTEM_VERSION));
    headers.insert("X-Sdk-Locale", HeaderValue::from_static(SDK_LOCALE));
    headers.insert("X-App-Id", HeaderValue::from_static(APP_ID));
    headers.insert("X-App-Version", HeaderValue::from_static(APP_VERSION));
    headers.insert("X-App-Code", HeaderValue::from_static(APP_CODE));
    headers.insert("X-Api-Version", HeaderValue::from_static(API_VERSION));
    headers.insert(
        "X-App-Device",
        HeaderValue::from_str(&device).map_err(|_| "invalid iOS device header".to_string())?,
    );
    headers.insert(
        "X-App-Token",
        HeaderValue::from_str(&token).map_err(|_| "invalid iOS token header".to_string())?,
    );

    Ok(request.headers(headers))
}

fn md5_hex(bytes: &[u8]) -> String {
    format!("{:x}", Md5::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ID: &str = "A1B2C3D4-E5F6-4789-8ABC-DEF012345678";

    #[test]
    fn app_device_matches_reference_mobile_source() {
        assert_eq!(
            app_device_for_profile(TEST_ID, "iPhone 15", "iPhone15,4", "MTLD3CH/A"),
            "E0LIN0MExEVN9CNsUTMl52boBVagsTNxASZu9GaQlGI7UGbwBXQgsTZsBHcBByOgsDI7AyO4cjN1QzMyEDMGVERtMkQBhTL5gzN00iNGVTRtQDRzMkMCFTQ"
        );
    }

    #[test]
    fn token_matches_reference_mobile_source() {
        assert_eq!(
            app_token(TEST_ID, 1_700_000_000),
            "6ff6898e9469e3fe8ce07b390f0e523bA1B2C3D4-E5F6-4789-8ABC-DEF0123456780x6553f100"
        );
    }

    #[test]
    fn user_agent_uses_ios_market_marker_and_version() {
        let ua = default_user_agent();
        assert!(ua.contains("+iCoolMarket/16.6.1-2610151"));
        assert!(!ua.contains("-universal"));
    }

    #[test]
    fn apply_headers_replaces_duplicate_profile_headers() {
        let request = reqwest::Client::new()
            .get("https://api.coolapk.com/v6/main/init")
            .header("X-App-Token", "stale-token")
            .header("X-Requested-With", "duplicate")
            .header("X-App-Id", "com.coolapk.market")
            .header("Accept-Language", "en-US");
        let request = apply_headers(request, TEST_ID).unwrap().build().unwrap();

        for name in [
            "Accept",
            "Accept-Language",
            "User-Agent",
            "X-Requested-With",
            "X-Sdk-Int",
            "X-Sdk-Locale",
            "X-App-Id",
            "X-App-Version",
            "X-App-Code",
            "X-Api-Version",
            "X-App-Device",
            "X-App-Token",
        ] {
            assert_eq!(
                request.headers().get_all(name).iter().count(),
                1,
                "header {name} must appear exactly once"
            );
        }
        assert_eq!(request.headers()["X-App-Id"], APP_ID);
        assert_eq!(request.headers()["X-Requested-With"], "XMLHttpRequest");
        assert_eq!(request.headers()["Accept-Language"], ACCEPT_LANGUAGE);
        assert_eq!(request.headers()["X-App-Device"], app_device(TEST_ID));
        assert!(!request.headers()["X-App-Token"].to_str().unwrap().contains("stale-token"));
    }
}