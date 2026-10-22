//! 酷安 iOS 客户端 Token 签名器。
//!
//! 原 Android v3 bcrypt 签名链路在 2026 年风控升级后失效（-415 / 验证码拦截）。
//! 这里改为 iOS 客户端签名：token 只依赖「设备 ID + 秒级时间戳」，与 c001apk
//! 的 `IosHeaders.appToken` 逐字节一致。
//!
//! 保留 `CoolapkAuth` 类型名是为了让 `client.rs` 的调用点（`get_app_token()`、
//! `set_device_code()`）尽量少改。设备 ID 语义已从「编码后的 X-App-Device 串」
//! 改为「原始 36 位 UUID（或登录后的 CDID）」。

use super::ios_identity::{self, IosDevice};

pub struct CoolapkAuth {
    device: IosDevice,
}

impl CoolapkAuth {
    /// `device_id` 是原始设备 ID（36 位 UUID 或 CDID）。
    pub fn new(device_id: impl Into<String>) -> Self {
        let mut device = IosDevice::default();
        device.id = device_id.into();
        Self { device }
    }

    /// 运行时切换设备 ID（登录/切换账号/游客态之间切换时调用）。
    pub fn set_device_code(&mut self, device_id: String) {
        self.device.id = device_id;
    }

    /// 生成当前秒级时间戳对应的 iOS `X-App-Token`。
    pub fn get_app_token(&self) -> Result<String, String> {
        Ok(ios_identity::app_token(&self.device, ios_identity::now_secs()))
    }

    /// 暴露当前设备档案，供请求层同时计算 `X-App-Device` / `User-Agent`。
    pub fn device(&self) -> &IosDevice {
        &self.device
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_starts_with_md5_hex_and_contains_device_id() {
        let auth = CoolapkAuth::new("A1B2C3D4-E5F6-4789-8ABC-DEF012345678");
        let token = auth.get_app_token().unwrap();
        // md5 hex 是 32 个十六进制字符，后面紧跟 device id 和 0x... 后缀。
        assert!(token.len() > 32 + 36 + 10, "token too short: {token}");
        assert!(token[..32].chars().all(|c| c.is_ascii_hexdigit()));
        assert!(token.contains("A1B2C3D4-E5F6-4789-8ABC-DEF012345678"));
        assert!(token.contains("0x"));
    }
}
