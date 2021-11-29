# Simple2FA

simple2fa is a library to easily add two-factor authentication to your app.

```rust
// Save this update to the database
use simple2fa;

// This represnts a user in your database.
struct User {
    pub otp_secret: String,
}

fn main() {
    let user = User {
        otp_secret: simple2fa::generate_secret()
    };

    // The QR code is a data url, so you can render it inline on a web page.
    // You can also use `create_png_qrcode` to generate a png file.
    let qrcode = simple2fa::create_urlencoded_qrcode("My web app", "Marie Curie", user.otp_secret)
    println!(r#"<!-- index.html -->
    <img src="{}"/>
    "#, qrcode);
    
    // Ask the user to scan the QR code with their phone, and then submit a code to confirm they have setup 2FA.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>") {
        // 2FA is setup, and the code is valid.
    } else {
        // Something went wrong setting up 2FA. The user might not be using a standards compliant authenticator app.
    }

    // When the user logs in, once you validate their password, respond with the user_id as a hidden 
    // input field and ask them to enter their 2FA code.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>") {
        // Log in successful
    } else {
        // Log in failed. Ask the user to try entering another otp code or reject their login attempt.
    }
```

# Installation

Add `simple2fa` to your `Cargo.toml`:

    [dependencies]
    simple2fa = "0.1.0"

If you have `cargo-edit`, you can use the command line:

    cargo add simple2fa

# Other languages