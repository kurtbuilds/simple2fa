use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha1, DEFAULT_STEP, totp_custom};
use rand::Rng;
use base32;
use base32::Alphabet;
use qrcode::QrCode;
use image::{DynamicImage, Luma};
use base64;
use digest::{BlockInput, FixedOutputDirty, Reset, Update};
use hmac::{Hmac, Mac, NewMac};


static DEFAULT_DIGITS: u32 = 6;
static DEFAULT_ALPHABET: Alphabet = Alphabet::RFC4648 { padding: false };
static DEFAULT_PERIOD: u64 = 30;


// fn to_bytes(n: u64) -> [u8; 8] {
//     let mask = 0x00000000000000ff;
//     let mut bytes: [u8; 8] = [0; 8];
//     (0..8).for_each(|i| bytes[7 - i] = (mask & (n >> (i * 8))) as u8);
//     bytes
// }
//
//
// pub fn totp_custom<H>(step: u64, digits: u32, secret: &[u8], time: u64) -> String
//     where
//         H: Update + BlockInput + Reset + FixedOutputDirty + Clone + Default,
// {
//     // Hash the secret and the time together.
//     let mut mac: Hmac<H> = Hmac::new_from_slice(secret).unwrap();
//     mac.update(&to_bytes(time / step));
//     let hash: &[u8] = &mac.finalize().into_bytes();
//
//     // Magic from the RFC.
//     let offset: usize = (hash.last().unwrap() & 0xf) as usize;
//     let binary: u64 = (((hash[offset] & 0x7f) as u64) << 24)
//         | ((hash[offset + 1] as u64) << 16)
//         | ((hash[offset + 2] as u64) << 8)
//         | (hash[offset + 3] as u64);
//
//     format!("{:01$}", binary % (10_u64.pow(digits)), digits as usize)
//     // let len = digits as usize;
//     // let mut s = format!("{1:0$}", len, binary);
//     // s.truncate(len);
//     // s
// }



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
    let bytes = base32::decode(DEFAULT_ALPHABET, &secret)
        .expect("Invalid secret. Secret must be RFC4686 base32 encoded, and should be 32 characters long for a 160 bit secret.");
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - DEFAULT_PERIOD;
    // check current time first for a small speed boost! very clever!
    vec![DEFAULT_PERIOD, 0, DEFAULT_PERIOD * 2].into_iter().any(|offset|
        totp_custom::<Sha1>(DEFAULT_STEP, DEFAULT_DIGITS, &bytes, seconds + offset) == code
    )
}


pub fn generate_2fa_code(secret: &str) -> String {
    let bytes = base32::decode(DEFAULT_ALPHABET, &secret)
        .expect("Invalid secret. Secret must be RFC4686 base32 encoded, and should be 32 characters long for a 160 bit secret.");
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
        assert!(4000 < len && len < 6500, "URL length is not in the correct range: {}", len);
    }

    #[test]
    fn test_generate_2fa_code() {
        let secret = generate_2fa_secret();
        println!("codee: {}", generate_2fa_code(&secret));
        assert!(check_2fa_code(&secret, &generate_2fa_code(&secret)));
    }
}
