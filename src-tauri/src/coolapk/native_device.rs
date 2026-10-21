//! 默认设备指纹：伪装为 iOS（iCoolMarket）客户端。
//!
//! 酷安 2025 年后服务端要求 iOS 风格的请求头（iCoolMarket UA + com.coolapk.app），
//! 桌面端与 Android 移动端统一伪装为同一组 iPhone 硬件信息，与 c001apk-next V1.0.8 一致。
use super::client::DeviceProfile;

/// 默认使用的 iPhone 硬件组合（随机池中的一项，固定以保证设备码持久稳定）。
pub const IOS_MODEL: &str = "iPhone 16";
pub const IOS_HARDWARE_ID: &str = "iPhone17,3";
pub const IOS_SALES_MODEL: &str = "MYEV3CH/A";
pub const IOS_SYSTEM_VERSION: &str = "26.7";

/// 官方 iCoolMarket UA 模板。
pub const COMPATIBILITY_UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 (#Build; Apple; iPhone17,3; iPhone 16; iOS_26.7) +iCoolMarket/16.6.1-2610151";

#[cfg(target_os = "android")]
pub fn detect() -> DeviceProfile {
    // 即使在 Android 平台也统一伪装为 iOS，避免读取真实设备属性导致指纹不一致。
    ios_compatibility_profile()
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub fn detect() -> DeviceProfile {
    ios_compatibility_profile()
}

#[cfg(not(any(target_os = "android", target_os = "ios", target_os = "macos")))]
pub fn detect() -> DeviceProfile {
    ios_compatibility_profile()
}

fn ios_compatibility_profile() -> DeviceProfile {
    DeviceProfile {
        manufacturer: Some("Apple".into()),
        brand: Some("Apple".into()),
        model: Some(IOS_SALES_MODEL.into()),
        build: Some(IOS_MODEL.into()),
        sdk_int: Some(IOS_SYSTEM_VERSION.into()),
        user_agent: Some(COMPATIBILITY_UA.into()),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ios_profile_uses_compatible_ua() {
        let profile = ios_compatibility_profile();
        assert_eq!(profile.sdk_int.as_deref(), Some(IOS_SYSTEM_VERSION));
        assert!(profile.user_agent.unwrap().contains("+iCoolMarket/16.6.1-2610151"));
    }
}
