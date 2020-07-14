# fenrir

Locates wifi devices using services such as wigle.net

[![Crates.io](https://img.shields.io/crates/v/fenrir.svg)](https://crates.io/crates/fenrir)
[![Build Status](https://travis-ci.org/wisespace-io/fenrir.png?branch=master)](https://travis-ci.org/wisespace-io/fenrir)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[Documentation](https://docs.rs/crate/fenrir/)

## Wigle.net api

After creating an account on wigle.net, visit https://wigle.net/account to get your API Token.
It correspondes to the "Encoded for use" field.

```rust
use async_std::task;
use fenrir::api::*;
use fenrir::wigle::api::*;

fn main() -> Result<(), surf::Exception> {
    let token = std::env::var("WIGLE_TOKEN").expect("Provide your WIGLE_TOKEN as an environment variable");
    task::block_on(async {
        let wigle: Wigle = Fenrir::new(Some(token));
        let geo_response = wigle.geocode("1600 Amphitheatre Parkway").await?;
        dbg!(geo_response);

        let search_response = wigle.search_bssid("00:00:00:00:00:00").await?;
        dbg!(search_response);

        Ok(())
    })
}
```

## Mylnikov api

It does not require an API token, it is completely free.

```rust
use async_std::task;
use fenrir::api::*;
use fenrir::mylnikov::api::*;

fn main() -> Result<(), surf::Exception> {
    task::block_on(async {
        let mylnikov: Mylnikov = Fenrir::new(None);
        let search_response = mylnikov.search_bssid("00:00:00:00:00:00").await?;
        dbg!(search_response);

        Ok(())
    })
}
```
