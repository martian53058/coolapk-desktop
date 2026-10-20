//! 酷安 Token 签名器。
//!
//! 2025 年后酷安将客户端伪装为 iOS（iCoolMarket），Token 算法由旧版 v3 bcrypt
//! 切换为基于 MD5 + Base64 的轻量方案：
//! ```text
//! part1 = md5_hex(unix_seconds)
//! raw   = "token://com.coolapk.app/1e7870f7312eb435aef6b59b211a9880?" + part1 + "$" + device_id + "&com.coolapk.app"
//! v9    = md5_hex( base64(raw) )
//! token = v9 + device_id + "0x" + hex(unix_seconds)
//! ```
//! 该算法与 c001apk-next V1.0.8 中 `f0.f.b()` 完全一致。
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use md5::{Digest, Md5};
use std::time::{SystemTime, UNIX_EPOCH};

const TOKEN_LABEL: &str = "token://com.coolapk.app/1e7870f7312eb435aef6b59b211a9880?";
const APP_ID: &str = "com.coolapk.app";

fn md5_hex(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// 酷安新版 Token 签名器（iOS 伪装方案）。
///
/// 与设备码（UUID 大写格式）绑定；切换账号/游客态时调用 [`CoolapkAuth::set_device_code`]。
pub struct CoolapkAuth {
    device_code: String,
}

impl CoolapkAuth {
    pub fn new(device_code: impl Into<String>) -> Self {
        Self { device_code: device_code.into() }
    }

    /// 运行时切换设备码（登录/切换账号/游客态之间切换时调用）。
    pub fn set_device_code(&mut self, device_code: String) {
        self.device_code = device_code;
    }

    /// 生成当前时间戳对应的 X-App-Token。
    pub fn get_app_token(&self) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("system clock error: {e}"))?
            .as_secs();
        Ok(self.generate_token_at(timestamp))
    }

    fn generate_token_at(&self, timestamp: u64) -> String {
        let part1 = md5_hex(timestamp.to_string().as_bytes());
        let raw = format!("{TOKEN_LABEL}{part1}${}&{APP_ID}", self.device_code);
        let base64_raw = BASE64.encode(raw.as_bytes());
        let v9 = md5_hex(base64_raw.as_bytes());
        format!("{v9}{}0x{timestamp:x}", self.device_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_deterministic_for_a_fixed_device_and_timestamp() {
        let auth = CoolapkAuth::new("A1B2C3D4-E5F6-7890-ABCD-EF1234567890");
        let first = auth.generate_token_at(1_792_480_360);
        let second = auth.generate_token_at(1_792_480_360);
        assert_eq!(first, second);
        // 结构: md5hex(32) + uuid(36) + "0x" + hex(ts)
        assert!(first.starts_with(|c: char| c.is_ascii_hexdigit()));
        assert!(first.contains("A1B2C3D4-E5F6-7890-ABCD-EF1234567890"));
        assert!(first.contains("0x"));
    }
}
