# scuffle-pprof

> [!WARNING]  
> This crate is under active development and may not be stable.

[![crates.io](https://img.shields.io/crates/v/scuffle-pprof.svg)](https://crates.io/crates/scuffle-pprof) [![docs.rs](https://img.shields.io/docsrs/scuffle-pprof)](https://docs.rs/scuffle-pprof)

---

A crate designed to provide a more ergonomic interface to the `pprof` crate.

## Example

```rust,no_run
// Create a new CPU profiler with a sampling frequency of 1000 Hz and an empty ignore list.
let cpu = scuffle_pprof::Cpu::new::<String>(1000, &[]);

// Capture a pprof profile for 10 seconds.
// This call is blocking. It is recommended to run it in a separate thread.
let capture = cpu.capture(std::time::Duration::from_secs(10)).unwrap();

// Write the profile to a file.
std::fs::write("capture.pprof", capture).unwrap();
```

## Analyzing the profile

The resulting profile can be analyzed using the [`pprof`](https://github.com/google/pprof) tool.

For example, to generate a flamegraph:

```sh
pprof -svg capture.pprof
```

## Status

This crate is currently under development and is not yet stable.

Unit tests are not yet fully implemented. Use at your own risk.

## License

This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
