#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use bincode_reloaded::{Decode, Encode};
use std::borrow::Cow;
use std::string::String;

#[derive(Decode, Encode, PartialEq, Debug)]
#[bincode_reloaded(
    decode_context = "()",
    borrow_decode_bounds = "&'__de U<'a, A>: ::bincode_reloaded::de::BorrowDecode<'__de, ()> + '__de, '__de: 'a"
)]
struct T<'a, A: Clone + Encode + Decode<()>> {
    t: Cow<'a, U<'a, A>>,
}

#[derive(Clone, Decode, Encode, PartialEq, Debug)]
#[bincode_reloaded(
    decode_context = "()",
    borrow_decode_bounds = "&'__de A: ::bincode_reloaded::de::BorrowDecode<'__de, ()> + '__de, '__de: 'a"
)]
struct U<'a, A: Clone + Encode + Decode<()>> {
    u: Cow<'a, A>,
}

#[test]
fn test() {
    let u = U {
        u: Cow::Owned(String::from("Hello world")),
    };
    let t = T {
        t: Cow::Borrowed(&u),
    };
    let vec = bincode_reloaded::encode_to_vec(&t, bincode_reloaded::config::standard()).unwrap();

    let (decoded, len): (T<String>, usize) =
        bincode_reloaded::decode_from_slice(&vec, bincode_reloaded::config::standard()).unwrap();

    assert_eq!(t, decoded);
    assert_eq!(len, 12);
}
