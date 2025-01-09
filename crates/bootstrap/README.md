# scuffle-bootstrap

> [!WARNING]  
> This crate is under active development and may not be stable.

[![crates.io](https://img.shields.io/crates/v/scuffle-bootstrap.svg)](https://crates.io/crates/scuffle-bootstrap) [![docs.rs](https://img.shields.io/docsrs/scuffle-bootstrap)](https://docs.rs/scuffle-bootstrap)

---

A utility crate for creating binaries.

## Usage

```rust
/// Our global state
struct Global;

// Required by the signal service
impl scuffle_signal::SignalConfig for Global {}

impl scuffle_bootstrap::global::GlobalWithoutConfig for Global {
    async fn init() -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(Self))
    }
}

/// Our own custom service
struct MySvc;

impl scuffle_bootstrap::service::Service<Global> for MySvc {
    async fn run(self, _: Arc<Global>, _: scuffle_context::Context) -> anyhow::Result<()> {
        println!("running");

        // Do some work here

        // Wait for the context to be cacelled by the signal service
        ctx.done().await;
        Ok(())
    }
}

// This generates the main function which runs all the services
main! {
    Global {
        scuffle_signal::SignalSvc,
        MySvc,
    }
}
```

## License

This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
