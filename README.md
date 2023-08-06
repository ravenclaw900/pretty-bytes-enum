# Pretty Bytes Typed
A simple, no-dependencies crate for converting a number of bytes into a strongly-typed representation of the "prettified" version of those bytes.

Compatible with `serde`! Designed for serializing possibly large byte values into JSON and sending to JavaScript code, since JS doesn't support full `u64` values.

Inspired by/derived from https://github.com/sindresorhus/pretty-bytes.