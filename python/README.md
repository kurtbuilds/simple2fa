# Simple2FA

simple2fa is a library to easily add two-factor authentication to your app.

This code is meant to be a full description of how to use simple2fa.

```python
import simple2fa
from simple2fa import create_urlencoded_qrcode

# This represents a user in your database.
class User:
    name = Column(String)
    otp_secret = Column(String)

    
def main():
    # Save the user's OTP secret to the database.
    user = User(
        name = "Marie Curie",
        otp_secret = simple2fa.generate_2fa_secret()
    )

    
    # Use this function or `create_png_qrcode` to generate a png file.
    qrcode_data_url = create_urlencoded_qrcode("My web app", user.name, user.otp_secret)
    
    
    # The QR code is a data url, so you can render it inline on a web page.
    print("""<!-- index.html -->
             <img src="{}"/>
    """, qrcode_data_url)

    
    # Ask the user to scan the QR code with an authenticator app,
    # and request a code to confirm they have setup 2FA.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>"):
        pass  # 2FA is setup!
    else:
        pass  # Something went wrong setting up 2FA. Have the user submit a code again.

    
    # When the user logs in, validate their password, then respond with
    # the user_id as a hidden input field and request a 2FA code.
    if simple2fa.check_2fa_code(user.otp_secret, "<otp_code>"):
        pass  # Log in successful
    else:
        pass  # Log in failed. Ask user re-enter an otp code or reject their login attempt.
        
if __name__ == '__main__':
    main()
```

# Installation

    pip3 install simple2fa

Simple2FA is also available in other languages:

<p align="center">
<a href="/kurtbuilds/simple2fa/tree/master/node">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/node.png?raw=true" width="96px"/>
</a>
<a href="/kurtbuilds/simple2fa/tree/master/python">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/python.png?raw=true" width="96px"/>
</a>
<a href="kurtbuilds/simple2fa/tree/master">
    <img src="https://github.com/kurtbuilds/logos/blob/9e56858d368da9e05a517c81ce28394f82d6b2fa/programming/rust.png?raw=true" width="96px"/>
</a>
</p>

# Benchmarks
    
Benchmarks are most easily run with `just bench` (See development section about `just`).

    The winner is: bench_simple2fa

    bench_simple2fa: Took 0.005ms per run.
    bench_pyotp: Took 0.129ms per run. (26.37x slower than the winner)

The benchmark runs the essential library functions as a single benchmark. In manual benchmarking,
individual functions of `simple2fa` outperform their counterparts between 12x and 106x.

Currently we benchmark against [pyotp](https://github.com/pyauth/pyotp). If there are other libraries
you'd like to see compared, we welcome pull requests!

# Development

Development commands are described in the `Justfile`.

# Appreciation

Thank you to:

- [@fosskers](https://github.com/fosskers) for the Rust [totp-lite](https://github.com/fosskers/totp-lite) library, 
    which this library depends on. `totp-lite` beat similar Rust libraries in benchmarking.
- The [neon](https://neon-bindings.com/) project, which makes creating Node libraries painless.
- The [pyo3](https://pyo3.rs/) project, which makes creating Python libraries painless.