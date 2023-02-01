use ordered_varint::Variable;
use rand::prelude::SliceRandom;
use rand::thread_rng;

fn main() {
    // Generate some random values
    let mut rng = thread_rng();
    let mut original_values = vec![];
    for power in 1..124 {
        original_values.push(2_u128.pow(power) - 1);
    }

    original_values.shuffle(&mut rng);

    // Encode the values.
    let mut encoded = original_values
        .iter()
        .map(|value| value.to_variable_vec().unwrap())
        .collect::<Vec<_>>();

    // Sort the original vec and the encoded vec.
    original_values.sort_unstable();
    encoded.sort();

    // Decode the encoded values.
    let decoded = encoded
        .iter()
        .map(|encoded| u128::decode_variable(encoded.as_slice()).unwrap())
        .collect::<Vec<_>>();

    // This assert proves that the encoded values are encoded such that sorting
    // works identially on the encoded and decoded values.
    assert_eq!(original_values, decoded);

    // Print some summary information
    let total_encoded_bytes: usize = encoded.iter().map(|encoded| encoded.len()).sum();
    let total_original_bytes = original_values.len() * std::mem::size_of::<u128>();
    println!("Original bytes: {}", total_original_bytes);
    println!("Encoded bytes: {}", total_encoded_bytes);
    for (original, encoded) in original_values.into_iter().zip(encoded.into_iter()) {
        println!("{} encodes as {:02x?}", original, encoded);
    }
}
