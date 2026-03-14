#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use std::collections::BTreeMap;

#[derive(bincode_reloaded::Encode)]
struct AllTypes(BTreeMap<u8, AllTypes>);

#[test]
fn test_issue_459() {
    let _result = bincode_reloaded::encode_to_vec(
        AllTypes(BTreeMap::new()),
        bincode_reloaded::config::standard(),
    );
}
