# reqwest-wasm-ext

[![crates.io](https://img.shields.io/crates/v/reqwest-wasm-ext.svg)](https://crates.io/crates/reqwest-wasm-ext)
[![docs.rs](https://docs.rs/reqwest-wasm-ext/badge.svg)](https://docs.rs/reqwest-wasm-ext)
[![CI](https://github.com/ctron/reqwest-wasm-ext/workflows/CI/badge.svg)](https://github.com/ctron/reqwest-wasm-ext/actions?query=workflow%3A%22CI%22)

This crate works around [seanmonstar/reqwest#1095](https://github.com/seanmonstar/reqwest/issues/1095).

Once [seanmonstar/reqwest#1096](https://github.com/seanmonstar/reqwest/pull/1096) is finally merged, this crate
should become obsolete.

## Example

Using the trait:

```rust
use reqwest::Client;
use reqwest_wasm_ext::RequestExt;

async fn do_request() {
    let client = Client::new();
    let req = client.get("http://localhost")
        .basic_auth_ext("foo", Some("bar"));
}
```

Using the drop-in replacement function:

```rust
use reqwest::Client;
#[cfg(target_arch = "wasm32")]
use reqwest_wasm_ext::RequestExt;

async fn do_request() {
    let client = Client::new();
    let req = client.get("http://localhost")
        .basic_auth("foo", Some("bar"));
}
```
