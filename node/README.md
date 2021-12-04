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
<a href="https://www.npmjs.com/package/simple2fa">
    <img src="https://img.shields.io/npm/dt/simple2fa?style=flat-square" alt="NPM" />
</a>
</p>

# Simple2FA

Simple2FA is a library to easily add two-factor authentication to your app.

```javascript
import * as simple2fa from 'simple2fa';

// save this user to the database
let user = { 
    name: "Marie Curie",
    otp_secret: simple2fa.generate_2fa_secret(),
};

// The QR code is a data url, so you can render it inline on your page. 
// You can also use `create_png_qrcode` to generate a png file.
let qrcode_data_url = simple2fa.create_urlencoded_qrcode("My web app", "Marie Curie", user.otp_secret)

console.log(`<!-- index.html -->
<img src="${qrcode_data_url}" />
`)

// Ask the user to scan the QR code with their phone, and then submit a code to confirm they have setup 2FA.
if (simple2fa.check_2fa_code(user.otp_secret, "<otp_code>")) {
    // 2FA is setup, and the code is valid.
} else {
    // Something went wrong setting up 2FA. Have the user submit a code again.
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

# Benchmarks

    The winner is: bench_simple2fa

    bench_simple2fa: Took 0.004140924565497097µs per run.
    bench_otplib: Took 0.0613251814689717µs per run.  (14.81x slower than the winner)

# Development

Development commands are described in the `Justfile`.

# Appreciation

Thank you to:

- [@fosskers](https://github.com/fosskers) for the Rust [totp-lite](https://github.com/fosskers/totp-lite) library, 
    which this library depends on. `totp-lite` beat similar Rust libraries in benchmarking.
- The [neon](https://neon-bindings.com/) project, which makes creating Node libraries painless.
- The [pyo3](https://pyo3.rs/) project, which makes creating Python libraries painless.
