#![cfg(feature = "derive")]

extern crate bincode_reloaded as bincode_new;

// Make sure that the `bincode_reloaded` crate exists, just symlink it to `core.
extern crate core as bincode_reloaded;

#[derive(bincode_new::Encode)]
#[bincode_reloaded(crate = "bincode_new")]
#[allow(dead_code)]
struct DeriveRenameTest {
    a: u32,
    b: u32,
}
