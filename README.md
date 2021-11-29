# Simple2FA

simple2fa is a library to easily add two-factor authentication to your app.

```rust
use simple2fa;
use simple2fa::create_url_encoded_qrcode;

// This represnts a user in your database.
struct User {
    pub otp_secret: String,
    pub name: String,
}

fn main() {
    // Save this to your database.
    let user = User {
        name: "Marie Curie".to_string(),
        otp_secret: simple2fa::generate_secret()
    };

    // You can also use `create_png_qrcode` to generate a png file.
    let qrcode_data_url = create_urlencoded_qrcode("My web app", user.name, user.otp_secret);
    // The QR code is a data url, so you can render it inline on a web page.
    println!(r#"<!-- index.html -->
    <img src="{}"/>
    "#, qrcode_data_url);

    // Ask the user to scan the QR code with an authenticator app, 
    // and request a code to confirm they have setup 2FA.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>") {
        // 2FA is setup!
    } else {
        // Something went wrong setting up 2FA. Have the user submit a code again.
    }

    // When the user logs in, validate their password, then respond with 
    // the user_id as a hidden input field and request a 2FA code.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>") {
        // Log in successful
    } else {
        // Log in failed. Ask user re-enter an otp code or reject their login attempt.
    }
```

# Installation

Add `simple2fa` to your `Cargo.toml`:

    [dependencies]
    simple2fa = "0.1.0"

If you have `cargo-edit`, you can use the command line:

    cargo add simple2fa

# Other languages