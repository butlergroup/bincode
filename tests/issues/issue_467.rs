#![cfg(all(feature = "std", feature = "derive"))]

extern crate std;

use std::collections::BTreeMap;

#[derive(bincode_reloaded::Decode, bincode_reloaded::Encode)]
struct AllTypes(BTreeMap<u8, AllTypes>);

#[test]
fn test_issue_467() {
    let _result: Result<(AllTypes, _), _> = bincode_reloaded::decode_from_slice(
        &[],
        bincode_reloaded::config::standard().with_limit::<1024>(),
    );
}
