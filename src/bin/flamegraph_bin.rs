use simple2fa::*;


fn main() {
    // let secret = generate_2fa_secret();
    let secret = generate_2fa_secret();
    check_2fa_code(&secret, "123456");
    // for _ in 0..10000 {
    //     // let code = generate_2fa_code(&secret);
    // }
}