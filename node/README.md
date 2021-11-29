# Simple2FA

Simple2FA is a library to easily add two-factor authentication to your app.

```javascript
import * as simple2fa from 'simple2fa';

let user = { otp_secret: null };
// Save this update to the database
user.otp_secret = simple2fa.generate_secret()

// The QR code is a data url, so you can render it inline on your page. 
// You can also use `create_png_qrcode` to generate a png file.
let qrcode = simple2fa.create_urlencoded_qrcode("My web app", "Marie Curie", user.otp_secret)

console.log(`<!-- index.html -->
<img src="${qrcode}" />
`)

// Ask the user to scan the QR code with their phone, and then submit a code to confirm they have setup 2FA.
if (simple2fa.check_2fa_code(user.otp_secret, "<otp_code>")) {
    // 2FA is setup, and the code is valid.
} else {
    // Something went wrong setting up 2FA. The user might not be using a standards compliant authenticator app.
}

// When the user logs in, once you validate their password, respond with the user_id as a hidden 
// input field and ask them to enter their 2FA code.
if (simple2fa.check_2fa_code(user.otp_secret, "<otp_code>")) {
    // Log in successful
} else {
    // Log in failed. Ask the user to try entering another otp code or reject their login attempt.
}

```

# Installation
        
    npm install -S simple2fa

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
