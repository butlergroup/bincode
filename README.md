# bincode_reloaded

<img align="right" src="./logo.svg" />

[<img alt="crates.io" src="https://img.shields.io/crates/v/bincode_reloaded.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bincode_reloaded)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bincode_reloaded-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bincode_reloaded)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![CodeQL](https://github.com/butlergroup/bincode_reloaded/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/github-code-scanning/codeql)
[![Rust CI/Unit Tests](https://github.com/butlergroup/bincode_reloaded/workflows/CI/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions)
[![Dependabot Updates](https://github.com/butlergroup/bincode_reloaded/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/dependabot/dependabot-updates)
[![CIFuzz](https://github.com/butlergroup/bincode_reloaded/actions/workflows/cifuzz.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/cifuzz.yml)
[![Cross platform tests](https://github.com/butlergroup/bincode_reloaded/actions/workflows/cross_platform.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/cross_platform.yml)
[![miri](https://github.com/butlergroup/bincode_reloaded/actions/workflows/miri.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/miri.yml)
[![rust-clippy analyze](https://github.com/butlergroup/bincode_reloaded/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/rust-clippy.yml)
[![Security audit](https://github.com/butlergroup/bincode_reloaded/actions/workflows/security.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/security.yml)
[![OSV-Scanner](https://github.com/butlergroup/bincode_reloaded/actions/workflows/osv-scanner.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/osv-scanner.yml)
[![Snyk Security-Monitored](https://img.shields.io/badge/Snyk%20Security-Monitored-purple)](https://app.snyk.io/share/784f6fef-6aaf-47ed-81ba-99e05b854665)
[![dependency status](https://deps.rs/repo/github/butlergroup/bincode_reloaded/status.svg)](https://deps.rs/repo/github/butlergroup/bincode_reloaded)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/12890/badge)](https://www.bestpractices.dev/projects/12890)
[![Scorecard supply-chain security](https://github.com/butlergroup/bincode_reloaded/actions/workflows/scorecard.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/scorecard.yml)
[![Microsoft Defender For Devops](https://github.com/butlergroup/bincode_reloaded/actions/workflows/defender-for-devops.yml/badge.svg)](https://github.com/butlergroup/bincode_reloaded/actions/workflows/defender-for-devops.yml)
[![Coverage Status](https://coveralls.io/repos/github/butlergroup/bincode_reloaded/badge.svg?branch=main)](https://coveralls.io/github/butlergroup/bincode_reloaded?branch=main)
[![Feature Requests](https://img.shields.io/github/issues/butlergroup/bincode_reloaded/feature-request.svg)](https://github.com/butlergroup/bincode_reloaded/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement)
[![Bugs](https://img.shields.io/github/issues/butlergroup/bincode_reloaded/bug.svg)](https://github.com/butlergroup/bincode_reloaded/issues?utf8=✓&q=is%3Aissue+is%3Aopen+label%3Abug)

A compact encoder / decoder pair that uses a binary zero-fluff encoding scheme.
The size of the encoded object will be the same or smaller than the size that
the object takes up in memory in a running Rust program.

In addition to exposing two simple functions
(one that encodes to `Vec<u8>`, and one that decodes from `&[u8]`),
binary-encode exposes a Reader/Writer API that makes it work
perfectly with other stream-based APIs such as Rust files, network streams,
and the [flate2-rs](https://github.com/rust-lang/flate2-rs) compression
library.

## Notes on this fork
 - dependencies in all Cargo.toml files updated to latest without build/unit test errors
 - originally forked from bincode 2.0.1
 - several security scanners have been added to the repo to ensure any issues are found quickly
 - MSRV (minimum supported Rust version) updated from 1.85 to 1.86 without build/unit test errors
 - Rust edition updated from 2021 to 2024 without build/unit test errors
 - minor code optimizations to improve efficiency
 - will be maintained (depenencies/crates updated & CVEs addressed in a timely manner, etc.)

## [API Documentation](https://docs.rs/bincode_reloaded/)

## bincode_reloaded in the Wild

* [google/tarpc](https://github.com/google/tarpc): bincode_reloaded is used to serialize and deserialize networked RPC messages.
* [servo/webrender](https://github.com/servo/webrender): bincode_reloaded records WebRender API calls for record/replay-style graphics debugging.
* [servo/ipc-channel](https://github.com/servo/ipc-channel): IPC-Channel uses bincode_reloaded to send structs between processes using a channel-like API.
* [ajeetdsouza/zoxide](https://github.com/ajeetdsouza/zoxide): zoxide uses bincode_reloaded to store a database of directories and their access frequencies on disk.

## Example

```rust
use bincode_reloaded::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let config = config::standard();

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = bincode_reloaded::encode_to_vec(&world, config).unwrap();

    // The length of the vector is encoded as a varint u64, which in this case gets collapsed to a single byte
    // See the documentation on varint for more info for that.
    // The 4 floats are encoded in 4 bytes each.
    assert_eq!(encoded.len(), 1 + 4 * 4);

    let (decoded, len): (World, usize) = bincode_reloaded::decode_from_slice(&encoded[..], config).unwrap();

    assert_eq!(world, decoded);
    assert_eq!(len, encoded.len()); // read all bytes
}
```

## Specification

bincode_reloaded's format is specified in [docs/spec.md](https://github.com/butlergroup/bincode_reloaded/blob/main/docs/spec.md).

## FAQ

### Is bincode_reloaded suitable for storage?

The encoding format is stable, provided the same configuration is used.
This should ensure that later versions can still read data produced by a previous versions of the library if no major version change
has occurred.

bincode_reloaded 1 and 2 are completely compatible if the same configuration is used.

bincode_reloaded is invariant over byte-order, making an exchange between different
architectures possible. It is also rather space efficient, as it stores no
metadata like struct field names in the output format and writes long streams of
binary data without needing any potentially size-increasing encoding.

As a result, bincode_reloaded is suitable for storing data. Be aware that it does not
implement any sort of data versioning scheme or file headers, as these
features are outside the scope of this crate.

### Is bincode_reloaded suitable for untrusted inputs?

bincode_reloaded attempts to protect against hostile data. There is a maximum size
configuration available (`Configuration::with_limit`), but not enabled in the
default configuration. Enabling it causes pre-allocation size to be limited to
prevent against memory exhaustion attacks.

Deserializing any incoming data will not cause undefined behavior or memory
issues, assuming that the deserialization code for the struct is safe itself.

bincode_reloaded can be used for untrusted inputs in the sense that it will not create a
security issues in your application, provided the configuration is changed to enable a
maximum size limit. Malicious inputs will fail upon deserialization.

### What is bincode_reloaded's MSRV (minimum supported Rust version)?

bincode_reloaded 2.0 has an MSRV of 1.86.0. Any changes to the MSRV are considered a breaking change for semver purposes, except when certain features are enabled. Features affecting MSRV are documented in the crate root.

### Why does bincode_reloaded not respect `#[repr(u8)]`?

bincode_reloaded will encode enum variants as a `u32`. If you're worried about storage size, we can recommend enabling `Configuration::with_variable_int_encoding()`. This option is enabled by default with the `standard` configuration. In this case enum variants will almost always be encoded as a `u8`.

Currently we have not found a compelling case to respect `#[repr(...)]`. You're most likely trying to interop with a format that is similar-but-not-quite-bincode_reloaded. We only support our own protocol ([spec](https://github.com/butlergroup/bincode_reloaded/blob/main/docs/spec.md)).

If you really want to use bincode_reloaded to encode/decode a different protocol, consider implementing `Encode` and `Decode` yourself. `bincode_reloaded-derive` will output the generated implementation in `target/generated/bincode_reloaded/<name>_Encode.rs` and `target/generated/bincode_reloaded/<name>_Decode.rs` which should get you started.

## Terms of Service

Please read our [Terms of Service](https://github.com/butlergroup/bincode_reloaded/blob/main/terms-of-service.md) before using our software. Violators of these Terms are not supported by the community or contributors.

## Privacy Policy

Please also read our [Privacy Policy](https://github.com/butlergroup/bincode_reloaded/blob/main/privacy-policy.md) to understand how we handle your personal information. 

## Contact

Have questions or suggestions? Reach out to us at dev@butlergroup.net. Thank you and happy coding! :)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=butlergroup/bincode_reloaded&type=Date)](https://www.star-history.com/#butlergroup/bincode_reloaded&Date)