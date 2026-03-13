#![cfg(all(feature = "derive", feature = "std"))]
#![allow(dead_code)]

use bincode::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum TypeOfFile {
    Unknown = -1,
}
