//! 酷安 iOS 客户端身份与签名实现。
//!
//! 背景：酷安服务端在 2026 年加强了对 Android 第三方客户端的风控（-415 账号过多、
//! err_request_captcha_v2 验证码拦截等），原 Android v3 bcrypt token 链路已不可用。
//! 参考 c001apk 与 `FuckCoolapkToken-iOS` 逆向，改用 iOS 客户端身份：
//! - `X-App-Device`：把 `设备ID; ; ; ; Apple; Apple; 机型; 硬件ID/销售型号` 做标准 Base64
//!   后整串反转，再去掉反转后跑到开头的 `=` 填充；
//! - `X-App-Token`：只依赖设备 ID 与秒级时间戳（md5 链路），不依赖 versionCode / szlmId。
//!
//! 与 `IosHeaders.kt` / `IosDeviceUtils.kt` 逐字节对齐。

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use md5::{Digest, Md5};
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

/// iOS 包名，同时用于 `X-App-Id` 与 token 材料。
pub const BUNDLE_ID: &str = "com.coolapk.app";
pub const APP_VERSION: &str = "16.6.1";
pub const APP_CODE: &str = "2610151";
pub const API_VERSION: &str = "16";
pub const SDK_LOCALE: &str = "zh-HK";
pub const ACCEPT_LANGUAGE: &str = "zh-Hans-HK;q=1.0, zh-Hant-HK;q=0.9, en-HK;q=0.8";

/// iOS token 固定前缀（逆向自官方客户端，跨版本稳定）。
const TOKEN_PREFIX: &str = "token://com.coolapk.app/1e7870f7312eb435aef6b59b211a9880?";

/// 一套持久化的 iOS 机型档案。桌面端固定选一组，保证多次启动之间
/// `X-App-Device` / UA / `X-App-Token` 自洽。
#[derive(Clone, Debug)]
pub struct IosDevice {
    /// 原始设备 ID：未登录是 36 位大写 UUID；登录后可换成 CDID（48 字符 D 开头）。
    pub id: String,
    pub model: String,
    pub hardware_id: String,
    pub sales_model: String,
    pub system_version: String,
}

impl Default for IosDevice {
    fn default() -> Self {
        // 默认档案：iPhone 16 / iPhone17,3 / MYEV3CH/A / iOS 26.7
        Self {
            id: String::new(),
            model: "iPhone 16".to_string(),
            hardware_id: "iPhone17,3".to_string(),
            sales_model: "MYEV3CH/A".to_string(),
            system_version: "26.7".to_string(),
        }
    }
}

fn md5_hex(data: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(data);
    hex::encode(h.finalize())
}

/// 生成大写 UUID v4（8-4-4-4-12），与 Java `UUID.randomUUID().toString().uppercase()` 一致。
pub fn random_uuid() -> String {
    use std::time::SystemTime;
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    seed ^= COUNTER.fetch_add(1, Ordering::Relaxed).wrapping_mul(0x9E3779B97F4A7C15);
    seed ^= std::process::id() as u64;

    let mut bytes = [0u8; 16];
    // 简单的 splitmix64 填充两次，拿到 16 个随机字节。
    let mut s = seed;
    for chunk in bytes.chunks_mut(8) {
        s = s.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z = z ^ (z >> 31);
        chunk.copy_from_slice(&z.to_be_bytes());
    }
    // UUID v4 版本与变体位。
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    format!(
        "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    )
}

/// 生成 `X-App-Device`：标准 Base64 → 反转 → 去掉开头的 `=`。
pub fn app_device_header(device: &IosDevice) -> String {
    let identity = if device.sales_model.is_empty() {
        device.hardware_id.clone()
    } else {
        format!("{}/{}", device.hardware_id, device.sales_model)
    };
    // 与 Kotlin 端 `joinToString("; ")` 完全一致：前 4 段是空字符串。
    let plain = format!(
        "{}; ; ; ; Apple; Apple; {}; {}",
        device.id, device.model, identity
    );
    let b64 = BASE64.encode(plain.as_bytes());
    let reversed: String = b64.chars().rev().collect();
    reversed.trim_start_matches('=').to_string()
}

/// iOS 客户端 User-Agent。
pub fn user_agent(device: &IosDevice) -> String {
    format!(
        "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 (#Build; Apple; {}; {}; iOS_{}) +iCoolMarket/{}-{}",
        device.hardware_id, device.model, device.system_version, APP_VERSION, APP_CODE
    )
}

/// 生成 iOS `X-App-Token`。
///
/// 末尾 `0x` 后是时间戳低 32 位十六进制；`timestamp_secs` 用秒级 Unix 时间。
pub fn app_token(device: &IosDevice, timestamp_secs: u64) -> String {
    let time_md5 = md5_hex(timestamp_secs.to_string().as_bytes());
    let material = format!(
        "{}{}${}&{}",
        TOKEN_PREFIX, time_md5, device.id, BUNDLE_ID
    );
    let digest = md5_hex(BASE64.encode(material.as_bytes()).as_bytes());
    format!("{}{}0x{:x}", digest, device.id, timestamp_secs as u32)
}

/// 一次生成全套 iOS 请求头（不含 Cookie）。
pub fn build_ios_headers(device: &IosDevice, timestamp_secs: u64) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", HeaderValue::from_static(ACCEPT_LANGUAGE));
    if let Ok(ua) = HeaderValue::from_str(&user_agent(device)) {
        headers.insert(USER_AGENT, ua);
    }
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
    if let Ok(v) = HeaderValue::from_str(&device.system_version) {
        headers.insert("X-Sdk-Int", v);
    }
    headers.insert("X-Sdk-Locale", HeaderValue::from_static(SDK_LOCALE));
    headers.insert("X-App-Id", HeaderValue::from_static(BUNDLE_ID));
    headers.insert("X-App-Version", HeaderValue::from_static(APP_VERSION));
    headers.insert("X-App-Code", HeaderValue::from_static(APP_CODE));
    headers.insert("X-Api-Version", HeaderValue::from_static(API_VERSION));
    if let Ok(v) = HeaderValue::from_str(&app_device_header(device)) {
        headers.insert("X-App-Device", v);
    }
    if let Ok(v) = HeaderValue::from_str(&app_token(device, timestamp_secs)) {
        headers.insert("X-App-Token", v);
    }
    headers
}

/// 当前秒级时间戳。
pub fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn golden_device() -> IosDevice {
        IosDevice {
            id: "A1B2C3D4-E5F6-4789-8ABC-DEF012345678".to_string(),
            model: "iPhone 15".to_string(),
            hardware_id: "iPhone15,4".to_string(),
            sales_model: "MTLD3CH/A".to_string(),
            system_version: "26.7".to_string(),
        }
    }

    #[test]
    fn app_device_matches_golden() {
        let got = app_device_header(&golden_device());
        let expected = "E0LIN0MExEVN9CNsUTMl52boBVagsTNxASZu9GaQlGI7UGbwBXQgsTZsBHcBByOgsDI7AyO4cjN1QzMyEDMGVERtMkQBhTL5gzN00iNGVTRtQDRzMkMCFTQ";
        assert_eq!(got, expected);
    }

    #[test]
    fn user_agent_matches_golden() {
        let got = user_agent(&golden_device());
        let expected = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 (#Build; Apple; iPhone15,4; iPhone 15; iOS_26.7) +iCoolMarket/16.6.1-2610151";
        assert_eq!(got, expected);
    }

    #[test]
    fn app_token_matches_golden() {
        let got = app_token(&golden_device(), 1_700_000_000);
        let expected = "6ff6898e9469e3fe8ce07b390f0e523bA1B2C3D4-E5F6-4789-8ABC-DEF0123456780x6553f100";
        assert_eq!(got, expected);
    }

    #[test]
    fn uuid_is_well_formed() {
        let id = random_uuid();
        assert_eq!(id.len(), 36);
        assert_eq!(id.chars().filter(|c| *c == '-').count(), 4);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
        assert!(id.chars().all(|c| !c.is_ascii_lowercase()));
    }
}
