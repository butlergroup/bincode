#![cfg(feature = "derive")]
#![allow(dead_code)]

#[derive(bincode::Encode, bincode::Decode)]
pub struct Eg<D, E> {
    data: (D, E),
}
