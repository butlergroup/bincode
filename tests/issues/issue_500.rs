#![cfg(all(feature = "serde", feature = "derive", feature = "std"))]

extern crate std;

type NodeId = u64;

use std::collections::BTreeSet;

#[derive(
    bincode_reloaded::Encode,
    bincode_reloaded::Decode,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    Debug,
    PartialEq,
    Eq,
)]
pub struct Membership {
    /// learners set
    learners: BTreeSet<NodeId>,
}

#[test]
fn test() {
    let mut start = Membership {
        learners: BTreeSet::new(),
    };
    start.learners.insert(1);

    let config = bincode_reloaded::config::legacy();
    let encoded = bincode_reloaded::encode_to_vec(&start, config).unwrap();
    std::dbg!(&encoded);
    let decoded: Membership = bincode_reloaded::serde::decode_from_slice(&encoded, config)
        .unwrap()
        .0;
    assert_eq!(start, decoded);
}
