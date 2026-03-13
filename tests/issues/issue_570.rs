#![cfg(feature = "derive")]
#![allow(dead_code)]

#[derive(bincode_reloaded::Encode, bincode_reloaded::Decode)]
pub struct Eg<D, E> {
    data: (D, E),
}
