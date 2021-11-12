# recaptcha-rs

> Recaptcha-rs is a very simple library to verify recaptcha responses.

This crate depends on `hyper==0.14` which is using `tokio 1` runtime. 

So keep in mind that you need `tokio 1` runtime when you want to use this crate.

## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)

**rust-toolchain**

```text
1.54.0
```

#### Importing

**Cargo.toml**

```toml
[dependencies]
recaptcha = { version = "0.1.0", git = "https://github.com/kumanote/recaptcha-rs", branch = "main" }
```

**rust files**

```rust
use recaptcha;
```

## Usage

```rust
use recaptcha;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote_ip = Some("123.123.123.123");
    let res = recaptcha::verify("your_private_key", "user_response", remote_ip).await;

    if res.is_ok() {
        println!("Success");
    } else {
        println!("Failure");
    }
    Ok(())
}
```
