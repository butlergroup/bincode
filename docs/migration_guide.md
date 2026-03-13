# Migrating from bincode_reloaded 1 to 2

bincode_reloaded 2 now has an optional dependency on `serde`. You can either use `serde`, or use bincode_reloaded's own `derive` feature and macros.

## From `Options` to `Configuration`

bincode_reloaded 1 had the [Options](https://docs.rs/bincode_reloaded/1/bincode_reloaded/config/trait.Options.html) trait. This has been replaced with the [Configuration](https://docs.rs/bincode_reloaded/2/bincode_reloaded/config/struct.Configuration.html) struct.

If you're using `Options`, you can change it like this:

```rust,ignore
# old
bincode_1::DefaultOptions::new().with_varint_encoding()

# new
bincode_2::config::legacy().with_variable_int_encoding()
```

If you want to be compatible with bincode_reloaded 1, use the following table:

| bincode_reloaded 1                                                              | bincode_reloaded 2                                       |
| ---------------------------------------------------------------------- | ----------------------------------------------- |
| version 1.0 - 1.2 with `bincode_1::DefaultOptions::new().serialize(T)` | `config::legacy()`                              |
| version 1.3+ with `bincode_1::DefaultOptions::new().serialize(T)`      | `config::legacy().with_variable_int_encoding()` |
| No explicit `Options`, e.g. `bincode_reloaded::serialize(T)`                    | `config::legacy()`                              |

If you do not care about compatibility with bincode_reloaded 1, we recommend using `config::standard()`

The following changes have been made:

- `.with_limit(n)` has been changed to `.with_limit::<n>()`.
- `.with_native_endian()` has been removed. Use `.with_big_endian()` or `with_little_endian()` instead.
- `.with_varint_encoding()` has been renamed to `.with_variable_int_encoding()`.
- `.with_fixint_encoding()` has been renamed to `.with_fixed_int_encoding()`.
- `.reject_trailing_bytes()` has been removed.
- `.allow_trailing_bytes()` has been removed.
- You can no longer (de)serialize from the `Options` trait directly. Use one of the `encode_` or `decode_` methods.

Because of confusion with `Options` defaults in bincode_reloaded 1, we have made `Configuration` mandatory in all calls in bincode_reloaded 2.

## Migrating with `serde`

You may wish to stick with `serde` when migrating to bincode_reloaded 2, for example if you are using serde-exclusive derive features such as `#[serde(deserialize_with)]`.

If so, make sure to include bincode_reloaded 2 with the `serde` feature enabled, and use the `bincode_reloaded::serde::*` functions instead of `bincode_reloaded::*` as described below:

```toml
[dependencies]
bincode_reloaded = { version = "2.0", features = ["serde"] }

# Optionally you can disable the `derive` feature:
# bincode_reloaded = { version = "2.0", default-features = false, features = ["std", "serde"] }
```

Then replace the following functions: (`Configuration` is `bincode_reloaded::config::legacy()` by default)

| bincode_reloaded 1                                       | bincode_reloaded 2                                                                                                                       |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `bincode_reloaded::deserialize(&[u8])`                   | `bincode_reloaded::serde::decode_from_slice(&[u8], Configuration)`<br />`bincode_reloaded::serde::borrow_decode_from_slice(&[u8], Configuration)` |
| `bincode_reloaded::deserialize_from(std::io::Read)`      | `bincode_reloaded::serde::decode_from_std_read(std::io::Read, Configuration)`                                                            |
| `bincode_reloaded::deserialize_from_custom(BincodeRead)` | `bincode_reloaded::serde::decode_from_reader(Reader, Configuration)`                                                                     |
|                                                 |                                                                                                                                 |
| `bincode_reloaded::serialize(T)`                         | `bincode_reloaded::serde::encode_to_vec(T, Configuration)`<br />`bincode_reloaded::serde::encode_into_slice(T, &mut [u8], Configuration)`         |
| `bincode_reloaded::serialize_into(std::io::Write, T)`    | `bincode_reloaded::serde::encode_into_std_write(T, std::io::Write, Configuration)`                                                       |
| `bincode_reloaded::serialized_size(T)`                   | Currently not implemented                                                                                                       |

## Migrating from `serde` to `bincode_reloaded-derive`

`bincode_reloaded-derive` is enabled by default. If you're using `default-features = false`, make sure to add `features = ["derive"]` to your `Cargo.toml`.

```toml,ignore
[dependencies]
bincode_reloaded = "2.0"

# If you need `no_std` with `alloc`:
# bincode_reloaded = { version = "2.0", default-features = false, features = ["derive", "alloc"] }

# If you need `no_std` and no `alloc`:
# bincode_reloaded = { version = "2.0", default-features = false, features = ["derive"] }
```

Replace or add the following attributes. You are able to use both `serde-derive` and `bincode_reloaded-derive` side-by-side.

| serde-derive                    | bincode_reloaded-derive               |
| ------------------------------- | ---------------------------- |
| `#[derive(serde::Serialize)]`   | `#[derive(bincode_reloaded::Encode)]` |
| `#[derive(serde::Deserialize)]` | `#[derive(bincode_reloaded::Decode)]` |

**note:** To implement these traits manually, see the documentation of [Encode](https://docs.rs/bincode_reloaded/2/bincode_reloaded/enc/trait.Encode.html) and [Decode](https://docs.rs/bincode_reloaded/2/bincode_reloaded/de/trait.Decode.html).

**note:** For more information on using `bincode_reloaded-derive` with external libraries, see [below](#bincode_reloaded-derive-and-libraries).

Then replace the following functions: (`Configuration` is `bincode_reloaded::config::legacy()` by default)

| bincode_reloaded 1                                       | bincode_reloaded 2                                                                                                          |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `bincode_reloaded::deserialize(&[u8])`                   | `bincode_reloaded::decode_from_slice(&bytes, Configuration)`<br />`bincode_reloaded::borrow_decode_from_slice(&[u8], Configuration)` |
| `bincode_reloaded::deserialize_from(std::io::Read)`      | `bincode_reloaded::decode_from_std_read(std::io::Read, Configuration)`                                                      |
| `bincode_reloaded::deserialize_from_custom(BincodeRead)` | `bincode_reloaded::decode_from_reader(Reader, Configuration)`                                                               |
|                                                 |                                                                                                                    |
| `bincode_reloaded::serialize(T)`                         | `bincode_reloaded::encode_to_vec(T, Configuration)`<br />`bincode_reloaded::encode_into_slice(t: T, &mut [u8], Configuration)`       |
| `bincode_reloaded::serialize_into(std::io::Write, T)`    | `bincode_reloaded::encode_into_std_write(T, std::io::Write, Configuration)`                                                 |
| `bincode_reloaded::serialized_size(T)`                   | Currently not implemented                                                                                          |

### bincode_reloaded derive and libraries

Currently not many libraries support the traits `Encode` and `Decode`. There are a couple of options if you want to use `#[derive(bincode_reloaded::Encode, bincode_reloaded::Decode)]`:

- Enable the `serde` feature and add a `#[bincode_reloaded(with_serde)]` above each field that implements `serde::Serialize/Deserialize` but not `Encode/Decode`
- Enable the `serde` feature and wrap your field in [bincode_reloaded::serde::Compat](https://docs.rs/bincode_reloaded/2/bincode_reloaded/serde/struct.Compat.html) or [bincode_reloaded::serde::BorrowCompat](https://docs.rs/bincode_reloaded/2/bincode_reloaded/serde/struct.BorrowCompat.html)
- Make a pull request to the library:
  - Make sure to be respectful, most of the developers are doing this in their free time.
  - Add a dependency `bincode_reloaded = { version = "2.0", default-features = false, optional = true }` to the `Cargo.toml`
  - Implement [Encode](https://docs.rs/bincode_reloaded/2/bincode_reloaded/enc/trait.Encode.html)
  - Implement [Decode](https://docs.rs/bincode_reloaded/2/bincode_reloaded/de/trait.Decode.html)
  - Make sure both of these implementations have a `#[cfg(feature = "bincode_reloaded")]` attribute.
