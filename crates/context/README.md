# scuffle-context

> [!WARNING]  
> This crate is under active development and may not be stable.

 [![crates.io](https://img.shields.io/crates/v/scuffle-context.svg)](https://crates.io/crates/scuffle-context) [![docs.rs](https://img.shields.io/docsrs/scuffle-context)](https://docs.rs/scuffle-context)

---

A crate designed to provide the ability to cancel futures using a context go-like approach, allowing for graceful shutdowns and cancellations.

## Why do we need this?

Its often useful to wait for all the futures to shutdown or to cancel them when we no longer care about the results. This crate provides an interface to cancel all futures associated with a context or wait for them to finish before shutting down. Allowing for graceful shutdowns and cancellations.

## Usage

Here is an example of how to use the `Context` to cancel a spawned task.

```rust
let (ctx, handler) = Context::new();

tokio::spawn(async {
    // Do some work
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}.with_context(ctx));

// Will stop the spawned task and cancel all associated futures.
handler.cancel();
```

## License

This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
