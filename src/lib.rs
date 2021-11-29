use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha1, totp_custom, DEFAULT_STEP};
use rand::Rng;
use base32;
use base32::Alphabet;
use qrcode::QrCode;
use image::{DynamicImage, Luma};
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
    let image = DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
    format!("data:image/png;base64,{}", base64::encode_config(&bytes, base64::URL_SAFE))
}


pub fn create_png_qrcode(service_name: &str, user_name: &str, secret: &str) -> Vec<u8> {
    let url = format!("otpauth://totp/{}:{}?secret={}&issuer={}", service_name, user_name, secret, service_name);
    let code = QrCode::new(url.as_bytes()).unwrap();
    let image = DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
    bytes
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
    fn test_urlencoded_qrcode() {
        let secret = generate_2fa_secret();
        let url = create_urlencoded_qrcode("My app", "Marie Curie", &secret);
        assert!(url.starts_with("data:image/png;base64,"), "URL did not start correctly: {}", url);
        let len = url.len();
        assert!(4000 < len && len < 6000, "URL length is not in the correct range: {}", len);
    }

    #[test]
    fn test_generate_2fa_code() {
        let secret = generate_2fa_secret();
        assert!(check_2fa_code(&secret, &generate_2fa_code(&secret)));
    }
}
