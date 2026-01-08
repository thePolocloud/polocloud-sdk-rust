# Adding the Dependency

Add the crate to your `Cargo.toml`
```toml
[dependencies]
polocloud-sdk = {git = "https://github.com/thePolocloud/polocloud-sdk-rust.git"}
```
Or if you want to use a specific branch
```toml
[dependencies]
polocloud-sdk = {git = "https://github.com/thePolocloud/polocloud-sdk-rust.git", branch = "master"}
```
---
# Example
File: ``Cargo.toml``
```toml
[package]
name = "project-name"
version = "0.1.0"
edition = "2024"

[dependencies]
polocloud-sdk = {git = "https://github.com/thePolocloud/polocloud-sdk-rust.git"}
tokio = "1.49.0"

```
File: ``main.rs``
```rust
use polocloud_sdk::init::polocloud;

#[tokio::main]
async fn main() {
    let polocloud = polocloud().await.unwrap();
    let group_provider = polocloud.group_provider();

    group_provider
        .lock()
        .unwrap()
        .find_all_async()
        .await
        .iter()
        .for_each(|group| {
            group.iter().for_each(|group| {
                let name = group.name();
                println!("Group: {:?}", group)
            });
        });
}

```
