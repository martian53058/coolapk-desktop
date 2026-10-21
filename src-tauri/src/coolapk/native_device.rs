//! Default device fingerprint matching the working c001apk_next Android client.
use super::client::DeviceProfile;
pub const APP_ID: &str = "com.coolapk.market";
pub const APP_VERSION: &str = "16.4.0";
pub const APP_CODE: &str = "2607021";
pub const API_VERSION: &str = "16";
pub const APP_CHANNEL: &str = "coolapk";
pub const APP_MODE: &str = "universal";
pub const SDK_INT: &str = "36";
pub const ANDROID_VERSION: &str = "16";
pub const MANUFACTURER: &str = "OnePlus";
pub const BRAND: &str = "OnePlus";
pub const MODEL: &str = "PJZ110";
pub const BUILD_NUMBER: &str = "PJZ110_16.0.5.703(CN01)";
pub const DEFAULT_DEVICE_CODE: &str = "lVDMjRWN2IzYjVTN3MmN2EDOiNWZjdjNhJTNkNTO3EWR4UzQzIkQ1EzN1YTMERTQFdjQ0cTMCF0QxYjNEJzM4AyOpEDMONEKzAzNuUjLw4iNx8FMxEjWKBFI7ATMxolSQByOzVHbQVmbPByOzVHbQVmbPByOgsDI7AyO3c2Xa9WaThFbxZTb0pXQplVW3FWMIJWWrZHZBdFNol1NMVFR";
pub const COMPATIBILITY_UA: &str = "Dalvik/2.1.0 (Linux; U; Android 16; PJZ110 PJZ110_16.0.5.703(CN01)) (#Build; OnePlus; PJZ110; PJZ110_16.0.5.703(CN01); 16) +CoolMarket/16.4.0-2607021-universal";
pub fn detect() -> DeviceProfile {
    DeviceProfile { manufacturer: Some(MANUFACTURER.into()), brand: Some(BRAND.into()), model: Some(MODEL.into()), build: Some(BUILD_NUMBER.into()), sdk_int: Some(SDK_INT.into()), locale: Some("zh-CN".into()), user_agent: Some(COMPATIBILITY_UA.into()), app_version: Some(APP_VERSION.into()), app_code: Some(APP_CODE.into()), api_version: Some(API_VERSION.into()), ..Default::default() }
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn profile_matches_reference_client_defaults() {
        let profile = detect(); assert_eq!(profile.manufacturer.as_deref(), Some("OnePlus")); assert_eq!(profile.model.as_deref(), Some("PJZ110"));
        assert!(profile.user_agent.unwrap().contains("+CoolMarket/16.4.0-2607021-universal"));
    }
}
