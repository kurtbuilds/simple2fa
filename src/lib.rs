use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha1, totp_custom, DEFAULT_STEP};
use rand::Rng;
use base32;
use base32::Alphabet;
use qrcode::QrCode;
use image::Luma;
use base64;

static DEFAULT_DIGITS: u32 = 6;
static DEFAULT_ALPHABET: Alphabet = Alphabet::RFC4648 { padding: false };
static DEFAULT_PERIOD: u64 = 30;

pub fn generate_2fa_secret() -> String {
    let bytes = rand::thread_rng().gen::<[u8; 20]>();
    base32::encode(DEFAULT_ALPHABET, &bytes)
}


pub fn create_urlencoded_qrcode(service_name: &str, user_name: &str, secret: &str) -> String {
    let url = format!("otpauth://totp/{}:{}?secret={}&issuer={}", service_name, user_name, secret, service_name);
    let code = QrCode::new(url.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    format!("data:image/png;base64,{}", base64::encode_config(&image.to_vec(), base64::URL_SAFE))
}


pub fn create_png_qrcode(service_name: &str, user_name: &str, secret: &str) -> Vec<u8> {
    let url = format!("otpauth://totp/{}:{}?secret={}&issuer={}", service_name, user_name, secret, service_name);
    let code = QrCode::new(url.as_bytes()).unwrap();
    code.render::<Luma<u8>>()
        .build()
        .to_vec()
}


pub fn check_2fa_code(secret: &str, code: &str) -> bool {
    let bytes = base32::decode(DEFAULT_ALPHABET, &secret).unwrap();
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - DEFAULT_PERIOD;
    // check current time first for a small speed boost! very clever!
    vec![DEFAULT_PERIOD, 0, DEFAULT_PERIOD * 2].into_iter().any(|offset|
        totp_custom::<Sha1>(DEFAULT_STEP, DEFAULT_DIGITS, &bytes, seconds + offset) == code
    )
}


pub fn generate_2fa_code(secret: &str) -> String {
    let bytes = base32::decode(DEFAULT_ALPHABET, &secret).unwrap();
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - DEFAULT_PERIOD;
    totp_custom::<Sha1>(DEFAULT_STEP, DEFAULT_DIGITS, &bytes, seconds)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_2fa_secret() {
        let secret = generate_2fa_secret();
        let secret2 = generate_2fa_secret();
        assert_eq!(secret.len(), 32);
        assert_ne!(secret, secret2);
    }

    #[test]
    fn test_generate_2fa_code() {
        let secret = generate_2fa_secret();
        assert!(check_2fa_code(&secret, &generate_2fa_code(&secret)));
    }
}
