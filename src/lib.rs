pub fn hello() -> String {
    format!("Hello Kurt")
}

pub fn generate_2fa_secret() -> String {
    unimplemented!()
}

pub fn create_urlencoded_qrcode(service_name: &str, user_name: &str, secret: &str) -> String {
    unimplemented!()
}

pub fn create_png_qrcode(service_name: &str, user_name: &str, secret: &str) -> Vec<u8> {
    unimplemented!()
}


pub fn check_2fa_code(secret: &str, code: &str) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
