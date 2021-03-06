<p align="center">
<a href="https://github.com/kurtbuilds/simple2fa/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/simple2fa.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/simple2fa/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/simple2fa.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/simple2fa/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/simple2fa/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/simple2fa">
    <img src="https://img.shields.io/crates/d/simple2fa?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/simple2fa">
    <img src="https://img.shields.io/crates/v/simple2fa?style=flat-square" alt="Crates.io" />
</a>
</p>

# Simple2FA


Simple2FA is a library to easily add two-factor authentication to your app.

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


    // Use this function or `create_png_qrcode` to generate a png file.
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
    // a form requesting a 2FA code, putting the user_id in JWT or similar server-signed
    // wrapper.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>") {
        // Log in successful
    } else {
        // Log in failed. Ask user re-enter an otp code or reject their login attempt.
    }
}
```

# Installation

Add `simple2fa` to your `Cargo.toml`:

    [dependencies]
    simple2fa = "0.1.0"

If you have `cargo-edit`, you can use the command line:

    cargo add simple2fa

Simple2FA is also available in other languages:

<p align="center">
<a href="/node">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/node.png?raw=true" width="96px"/>
</a>
<a href="/python">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/python.png?raw=true" width="96px"/>
</a>
<a href="https://github.com/kurtbuilds/simple2fa/">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/rust.png?raw=true" width="96px"/>
</a>
</p>

# Development

Development commands are described in the `Justfile`.

# Appreciation

Thank you to:

- [@fosskers](https://github.com/fosskers) for the Rust [totp-lite](https://github.com/fosskers/totp-lite) library,
  which this library depends on. `totp-lite` beat similar Rust libraries in benchmarking.
- The [neon](https://neon-bindings.com/) project, which makes creating Node libraries painless.
- The [pyo3](https://pyo3.rs/) project, which makes creating Python libraries painless.
