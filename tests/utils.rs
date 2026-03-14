use core::fmt::Debug;

fn the_same_with_config<V, C, CMP>(element: &V, config: C, cmp: CMP)
where
    V: TheSameTrait,
    C: bincode_reloaded::config::Config,
    CMP: Fn(&V, &V) -> bool,
{
    let mut buffer = [0u8; 2048];
    let len = bincode_reloaded::encode_into_slice(element, &mut buffer, config).unwrap();
    println!(
        "{:?} ({}): {:?} ({:?})",
        element,
        core::any::type_name::<V>(),
        &buffer[..len],
        core::any::type_name::<C>()
    );
    let (decoded, decoded_len): (V, usize) =
        bincode_reloaded::decode_from_slice(&buffer, config).unwrap();

    assert!(
        cmp(element, &decoded),
        "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
        decoded,
        element,
        &buffer[..len],
    );
    assert_eq!(len, decoded_len);

    #[cfg(feature = "serde")]
    the_same_with_config_serde(element, config, cmp)
}

#[cfg(feature = "serde")]
fn the_same_with_config_serde<V, C, CMP>(element: &V, config: C, cmp: CMP)
where
    V: TheSameTrait,
    C: bincode_reloaded::config::Config,
    CMP: Fn(&V, &V) -> bool,
{
    let mut buffer = [0u8; 2048];
    let len = bincode_reloaded::serde::encode_into_slice(element, &mut buffer, config);

    let decoded = bincode_reloaded::serde::decode_from_slice(&buffer, config);

    let len = len.unwrap();
    let (decoded, decoded_len): (V, usize) = decoded.unwrap();
    println!(
        "{:?} ({}): {:?} ({:?})",
        element,
        core::any::type_name::<V>(),
        &buffer[..len],
        core::any::type_name::<C>()
    );

    assert!(
        cmp(element, &decoded),
        "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
        decoded,
        element,
        &buffer[..len],
    );
    assert_eq!(len, decoded_len);
}

pub fn the_same_with_comparer<V, CMP>(element: V, cmp: CMP)
where
    V: TheSameTrait,
    CMP: Fn(&V, &V) -> bool,
{
    // A matrix of each different config option possible
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_little_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_big_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_little_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode_reloaded::config::standard()
            .with_big_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
}

#[cfg(feature = "serde")]
pub trait TheSameTrait:
    bincode_reloaded::Encode
    + bincode_reloaded::Decode<()>
    + serde::de::DeserializeOwned
    + serde::Serialize
    + Debug
    + 'static
{
}
#[cfg(feature = "serde")]
impl<T> TheSameTrait for T where
    T: bincode_reloaded::Encode
        + bincode_reloaded::Decode<()>
        + serde::de::DeserializeOwned
        + serde::Serialize
        + Debug
        + 'static
{
}

#[cfg(not(feature = "serde"))]
pub trait TheSameTrait:
    bincode_reloaded::Encode + bincode_reloaded::Decode<()> + Debug + 'static
{
}
#[cfg(not(feature = "serde"))]
impl<T> TheSameTrait for T where
    T: bincode_reloaded::Encode + bincode_reloaded::Decode<()> + Debug + 'static
{
}

#[allow(dead_code)] // This is not used in every test
pub fn the_same<V: TheSameTrait + PartialEq>(element: V) {
    the_same_with_comparer(element, |a, b| a == b);
}
