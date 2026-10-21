//! Coolapk Token V3 signer aligned with c001apk_next TokenDeviceUtils.getTokenV3.
use base64::{engine::general_purpose::{STANDARD, STANDARD_NO_PAD}, Engine as _};
use bcrypt::{hash_with_salt, Version, BASE_64 as BCRYPT_BASE64};
use md5::{Digest, Md5};
use std::time::{SystemTime, UNIX_EPOCH};

pub const APP_ID: &str = "com.coolapk.market";
pub const APP_VERSION: &str = "16.4.0";
pub const APP_CODE: &str = "2607021";
const KEY_B64: &str = "VFRCVU9GUXNRMEVsTFVNa1dERWpRU0VzVXlFbUxETkFVeTFEUEZZdUl5MGlNVU01SXpCVUpGY3RNeWtoTVVRMUpTMURPU1V4TXpoUUxWTXdWMDBzSXloU01UUXNXVEZEUkZFd1EwUlZNU1FrVVMwaktTTXRKRGhZTFZRa1VqRXpLU1FzTTBCVkxUUXdVaTAwTlNNc00yQlhMVk1sSkMxRE5GbE5MQ1EwV1RFekxTUXhRMEVoTEVNbElqRXpLRmN3TXowbExpTkVWekVqUFNZdU5Ea2tMRk13Vml4VEtGa3RRemhYTEZNc1dDNGpMU1l0VTJCVFRUQTBMRkV1TXpoVExqTWhJakZETUZNc0kwQlZMQ1FrVUN3ekpTSXNJekJTTUVNOFVDeEVMU0l0STJCUUxVUW9VeTBqTEZNc0l5VWtNRlF3VmswdFJDeFlNVE1sSkM0ME5TRXdVMFJRTEZNaElpMGtOU013VTJCUUxqTmdVeXdqT0ZFc05DeFRMRU13V0N4VUxTRXRJeVJUTEROQkl5MDBKRlpOTVVNc1VEQkVKRmd0TkMwa0xEUWtXQzFETUZNeFJDa2pMVk0wVml4VE1GZ3hJeTBtTFZNeEpTMHpKRlF4TTJCUUxTUTRXVEJETUZNdUkwVWhUU3d6UEZndFJEQlJNRE5FVVMwak9GWXVKRGtsTVNRMFV6QXpOU013UkRra01UTWtWU3hVTUZRdFJDeFlNVE5GSXl4RUxGWXNJelVoTGlRMFYwMHhOQ3hWTENNMFZ5NHpKRlV4TkN4VExGUTRWeTRqWUZZeEpEVWpMVU13VXpGRFBGSXNNMEVsTERNc1ZEQTBPU011TkNrbUxFUWtWaXhFTVNKTkxUTW9VaXd6TEZjdFUwUlFMaVF3VlRGREpTVXdVeWtqTVVNaEl5MHpJU1VzSTBFak1UTXdVUzFVTkZjeFJERWxMVVF0SlM0a05GSXNJemhaVFRCVFlGRXdReVJZTFNNbElTNDBMRmdzVkRSWkxVUW9XVEJEUUZZdFJDeFdMak5FVkRCRE1Ga3RORFVsTEZRNFZTMVVKRmd3TXpSV0xEUTRVVTB3UXkwbUxVUTRXUzFUTVNNc1V5VWlNVU5BVVN4VUxGWXVJMFJVTGlNc1dERkVMRk14STJCUUxWTTBXUzRrT0Zjd00wUlhMVk1vV0MwaktGRXhMQ00wVml4VE5GQXRSRFJXTFVRbEpDMVRKRkF3TkN4Z1lB";

fn md5_hex(data: &[u8]) -> String { let mut hasher = Md5::new(); hasher.update(data); hex::encode(hasher.finalize()) }

pub struct CoolapkAuth { device_code: String }
impl CoolapkAuth {
    pub fn new(device_code: impl Into<String>) -> Self { Self { device_code: device_code.into() } }
    pub fn set_device_code(&mut self, device_code: String) { self.device_code = device_code; }
    pub fn get_app_token(&self) -> Result<String, String> { self.get_app_token_for(APP_CODE) }
    pub fn get_app_token_for(&self, version_code: &str) -> Result<String, String> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| format!("system clock error: {e}"))?.as_secs();
        self.generate_token_for(timestamp, version_code)
    }
    fn generate_token_at(&self, timestamp: u64) -> Result<String, String> { self.generate_token_for(timestamp, APP_CODE) }
    fn generate_token_for(&self, timestamp: u64, version_code: &str) -> Result<String, String> {
        let vc = version_code.parse::<u64>().unwrap_or(2607021);
        let key_table = STANDARD.decode(KEY_B64).map_err(|e| format!("invalid Token V3 key table: {e}"))?;
        let offset = (4 * ((timestamp + vc) % 100) + 128) as usize;
        let key_slice = key_table.get(offset..offset + 128).ok_or("Token V3 key slice out of range")?;
        let b2 = STANDARD.decode(key_slice).map_err(|e| format!("invalid Token V3 key slice: {e}"))?;
        let device_md5 = md5_hex(self.device_code.as_bytes());
        let mut material = Vec::with_capacity(APP_ID.len() + b2.len() + device_md5.len() + 40);
        material.extend_from_slice(APP_ID.as_bytes()); material.push(b'&'); material.extend_from_slice(&b2); material.push(b'&');
        material.extend_from_slice(device_md5.as_bytes()); material.push(b'&'); material.extend_from_slice(timestamp.to_string().as_bytes());
        material.push(b'&'); material.extend_from_slice(vc.to_string().as_bytes());
        let salt_source = format!("{:x}/{}", timestamp, md5_hex(&material));
        let salt_b64 = STANDARD.encode(salt_source.as_bytes());
        let salt_text = &salt_b64[..22];
        // c001apk passes standard-Base64 text to jBCrypt, whose decoder uses bcrypt's alphabet.
        let salt_vec = BCRYPT_BASE64.decode(salt_text.as_bytes()).map_err(|e| format!("invalid Token V3 bcrypt salt: {e}"))?;
        let salt: [u8; 16] = salt_vec.try_into().map_err(|_| "Token V3 salt must be 16 bytes")?;
        let password = md5_hex(STANDARD.encode(&material).as_bytes());
        let hash = hash_with_salt(password.as_bytes(), 4, salt).map_err(|e| format!("Token V3 bcrypt failed: {e}"))?.format_for_version(Version::TwoY);
        Ok(format!("v3{}", STANDARD_NO_PAD.encode(hash.as_bytes())))
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn token_v3_is_deterministic_and_has_reference_format() {
        let auth = CoolapkAuth::new("lVDMjRWN2IzYjVTN3MmN2EDOiNWZjdjNhJTNkNTO3EWR4UzQzIkQ1EzN1YTMERTQFdjQ0cTMCF0QxYjNEJzM4AyOpEDMONEKzAzNuUjLw4iNx8FMxEjWKBFI7ATMxolSQByOzVHbQVmbPByOzVHbQVmbPByOgsDI7AyO3c2Xa9WaThFbxZTb0pXQplVW3FWMIJWWrZHZBdFNol1NMVFR");
        let first = auth.generate_token_at(1_700_000_000).unwrap();
        let second = auth.generate_token_at(1_700_000_000).unwrap();
        assert_eq!(first, second); assert!(first.starts_with("v3"));
        let decoded = STANDARD.decode(&first[2..]).unwrap();
        assert!(String::from_utf8(decoded).unwrap().starts_with("$2y$04$"));
    }
    #[test] fn token_uses_matching_version_code() {
        let auth = CoolapkAuth::new("sample-device");
        assert_ne!(auth.generate_token_for(1_700_000_000, "2607021").unwrap(), auth.generate_token_for(1_700_000_000, "2610151").unwrap());
    }
}
