use std::{env, fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

pub mod memory_manager;

// Function to parse data from the command line input file
pub fn parse_data(data: &str) -> Vec<u8> {
    if data.starts_with("0x") || data.starts_with("0X") {
        // Explicitly specify the type to collect to Vec<u8>
        data[2..]
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).expect("Failed to parse hex"))
            .collect::<Vec<u8>>()
    } else {
        data.as_bytes().to_vec()
    }
}

// Function to find the nearest power of two for a given size
pub fn nearest_power_of_two(size: usize) -> usize {
    let mut n = size - 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n += 1;
    n
}