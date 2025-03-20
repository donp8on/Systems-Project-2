// BS: strong start so far. I recommend implementing MemoryManager.dump() as soon as possible. This
// will make testing all of the other public methods of MemoryManager easier.
pub mod memory_manager;

// Function to parse data from the command line input file
fn parse_data(data: &str) -> Vec<u8> {
    if data.starts_with("0x") || data.starts_with("0X") {
        data[2..]
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect()
    } else {
        data.as_bytes().to_vec()
    }
}

// Function to find the nearest power of two for a given size
//
// BS: I believe you should be finding the smallest power of 2
// that is larger than `size`.
fn nearest_power_of_two(size: usize) -> usize {
    let mut power = 1;
    while power < size {
        power <<= 1;
    }
    power
}
