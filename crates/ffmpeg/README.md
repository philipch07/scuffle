# scuffle-ffmpeg

> [!WARNING]  
> This crate is under active development and may not be stable.

[![crates.io](https://img.shields.io/crates/v/scuffle-ffmpeg.svg)](https://crates.io/crates/scuffle-ffmpeg) [![docs.rs](https://img.shields.io/docsrs/scuffle-ffmpeg)](https://docs.rs/scuffle-ffmpeg)

---

A crate designed to provide a simple interface to the native ffmpeg c-bindings.

Currently this crate only supports the latest versions of ffmpeg (7.x.x)

## Why do we need this?

This crate aims to provide a simple-safe interface to the native ffmpeg c-bindings.

## How is this different from other ffmpeg crates?

The other main ffmpeg crate is [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg).

This crate adds a few features and has a safer API. Notably it adds the ability to provide a an in-memory decode / encode buffer.

## Examples

```rust
let input = scuffle_ffmpeg::io::Input::seekable(std::fs::File::open(path)?)?;
let streams = input.streams();

dbg!(&streams);

let best_video_stream = streams.best(AVMediaType::AVMEDIA_TYPE_VIDEO).expect("no video stream found");
let best_audio_stream = streams.best(AVMediaType::AVMEDIA_TYPE_AUDIO).expect("no audio stream found");

dbg!(&best_video_stream);
dbg!(&best_audio_stream);

let video_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_video_stream)?.video().expect("not an video decoder");
let audio_decoder = scuffle_ffmpeg::decoder::Decoder::new(&best_audio_stream)?.audio().expect("not an audio decoder");

dbg!(&video_decoder);
dbg!(&audio_decoder);
```

## Status

This crate is currently under development and is not yet stable.

Unit tests are not yet fully implemented. Use at your own risk.

## License

This project is licensed under the [MIT](./LICENSE.MIT) or [Apache-2.0](./LICENSE.Apache-2.0) license.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
