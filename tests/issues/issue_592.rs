#![cfg(all(feature = "derive", feature = "std"))]
#![allow(dead_code)]

use bincode_reloaded::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum TypeOfFile {
    Unknown = -1,
}
