use std::collections::HashMap;
mod memory_block;

use memory_block::alloacted;

use crate::MemoryBlock;
use crate::nearest_power_of_two;
use crate::parse_data;


// Define the memory manager
pub struct MemoryManager {
    memory: [u8; 65535],
    free_blocks: Vec<MemoryBlock>, // Using Vec to manage free blocks
    allocated_blocks: HashMap<u32, MemoryBlock>,
    next_id: u32,
}

// Implement the memory manager
impl MemoryManager {
    // Constructor to initialize the memory manager
    pub fn new() -> MemoryManager {
        MemoryManager {
            memory: [0; 65535],
            free_blocks: vec![MemoryBlock { start: 0, size: 65535, id: 0 }], // Initial large block
            allocated_blocks: HashMap::new(),
            next_id: 0,
        }
    }

    // Function to allocate memory blocks using a buddy system
    pub fn allocate(&mut self, requested_size: usize) -> Result<u32, String> {
        let size = nearest_power_of_two(requested_size);
        if let Some(index) = self.free_blocks.iter().position(|block| block.size >= size) {
            let block = self.free_blocks.remove(index);
            if block.size > size {
                self.free_blocks.push(MemoryBlock {
                    start: block.start + size,
                    size: block.size - size,
                    id: block.id,
                });
            }
            let allocated_block = MemoryBlock {
                start: block.start,
                size: size,
                id: self.next_id,
            };
            self.allocated_blocks.insert(self.next_id, allocated_block);
            let id = self.next_id;
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }

    // Function to set data in allocated memory blocks
    pub fn set(&mut self, id: u32, data: &[u8]) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            if data.len() <= block.size {
                let end = block.start + data.len();
                self.memory[block.start..end].copy_from_slice(data);
                Ok(())
            } else {
                Err(format!("Data size exceeds block size.\ndata size: {}\nblock size: {}", data.len(), block.size))
            }
        } else {
            Err("Block ID not found".to_string())
        }
    }

    // Function to insert data into allocated memory blocks
    pub fn insert(&mut self, size: usize) -> Result<u32, String> {
        let size = nearest_power_of_two(size);
        if let Some(index) = self.free_blocks.iter().position(|block| block.size >= size) {
            let block = self.free_blocks.remove(index);
    
            if block.size > size {
                self.free_blocks.push(MemoryBlock {
                    start: block.start + size,
                    size: block.size - size,
                    id: block.id,
                });
            }
    
            let allocated_block = MemoryBlock {
                start: block.start,
                size: size,
                id: self.next_id,
            };
    
            self.allocated_blocks.insert(self.next_id, allocated_block);
            let id = self.next_id;
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }
    
    

    // Function to read data from allocated memory blocks
    pub fn read(&self, id: u32) -> Result<alloacted, String> {
        if let Some(block) = self.allocated_blocks.get(&id) {
            let start_addr = block.start;
            let end_addr = block.start + block.size - 1;  // Calculate end address
            let size = block.size;  // This is the capacity of the block

            // Assuming you want to return the actual used data length separately,
            let data_length = block.size;  // Modify as needed based on actual usage data

            Ok(alloacted::new(start_addr, end_addr, size, data_length))
        } else {
            // Check if it's a free block, otherwise return error about missing block
            if let Some(block) = self.free_blocks.iter().find(|&b| b.id == id) {
                Err(format!("Block ID {} is FREE and cannot be read. Start Address: 0x{:04X}, Size: {} bytes", id, block.start, block.size))
            } else {
                Err(format!("Nothing at [{}]", id))
            }
        }
    }

    // Function to execute commands from the input file (.cmmd)
    pub fn execute_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts[0] {
            "INSERT" => {
                let size = parts[1].parse::<usize>().unwrap();
                let data = parse_data(&parts[2..].join(" "));
                if let Ok(id) = self.insert(size) {
                    match self.set(id, &data) {
                        Ok(_) => println!("INSERT success: ID = {}", id),
                        Err(e) => println!("Error storing data: {}", e),
                    }
                } else {
                    println!("INSERT error: No suitable block available");
                }
            },
            "READ" => {
                let id = parts[1].parse::<u32>().expect("Invalid ID format");
                match self.read(id) {
                    Ok(data) => {
                        println!("READ data: Start Address: {}, End Address: {}, Status: {}, Size: {}, Data Length: {}", 
                            data.0, data.1, data.2, data.3, data.4);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            },
            _ => println!("Unknown command"),
        }
    }
}