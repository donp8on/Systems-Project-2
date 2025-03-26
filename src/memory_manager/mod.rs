use crate::*;
use allocated_block::AllocatedBlock;
use free_block::FreeBlock;
use std::collections::HashMap;
use allocated_block::DataMemoryBlock;

mod allocated_block;
mod free_block;

// Define the memory manager
pub struct MemoryManager {
    memory: [u8; 65535],
    free_blocks: Vec<FreeBlock>, // Using Vec to manage free blocks
    allocated_blocks: HashMap<usize, AllocatedBlock>,
    next_id: usize,
}

// Implement the memory manager
impl MemoryManager {
    // Constructor to initialize the memory manager
    pub fn new() -> MemoryManager {
        MemoryManager {
            memory: [0; 65535],
            free_blocks: vec![FreeBlock::new(0, 65535)], // Initial large block
            allocated_blocks: HashMap::new(),
            next_id: 0,
        }
    }

    // Function to set data in allocated memory blocks
    //
    // BS: Remember to update AllocatedBlock.data_size in this method.
    // AP: I have updated the data_size in the AllocatedBlock struct.
    pub fn set(&mut self, id: usize, data: &[u8]) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            if data.len() <= block.get_size() {
                let end = block.get_start() + data.len();
                self.memory[block.get_start()..end].copy_from_slice(data);
                // Update the data_size to reflect the actual amount of data stored
                block.set_data_size(data.len());
                Ok(())
            } else {
                Err(format!(
                    "Data size exceeds block size.\ndata size: {}\nblock size: {}",
                    data.len(),
                    block.get_size()
                ))
            }
        } else {
            Err("Block ID not found".to_string())
        }
    }

    // Function to allocate memory blocks using a buddy system
    //
    // BS: this doesn't seem correct, but I may be misunderstanding. First,
    // you're finding the correct size to allocate, then you're searching
    // MemoryManager.free_blocks for a block that will fit. Up until here makes
    // sense.
    //
    // After you find a block, you remove it from MemoryManager.free_blocks and then add it back?

    // Function to insert data into allocated memory blocks using the buddy system
    pub fn insert(&mut self, data_size: usize) -> Result<usize, String> {
        let size = nearest_power_of_two(data_size);
        if let Some(index) = self
            .free_blocks
            .iter()
            .position(|block| block.get_size() >= size)
        {
            let block = self.free_blocks.remove(index);

            if block.get_size() > size {
                self.free_blocks.push(FreeBlock::new(
                    block.get_start() + size,
                    block.get_size() + size,
                ));
            }

            let allocated_block =
                AllocatedBlock::new(block.get_start(), size, self.next_id, data_size);

            let _ = self.allocated_blocks.insert(self.next_id, allocated_block);
            let id = self.next_id;
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }

    // Function to read data from allocated memory blocks
    pub fn read(&self, id: usize) -> Result<AllocatedBlock, String> {
        if let Some(block) = self.allocated_blocks.get(&id) {
            let start_addr = block.get_start();
            let end_addr = block.get_start() + block.get_size() - 1; // Calculate end address
            let size = block.get_size(); // This is the capacity of the block

            // Assuming you want to return the actual used data length separately,
            let data_length = block.get_size(); // Modify as needed based on actual usage data

            Ok(AllocatedBlock::new(start_addr, end_addr, size, data_length))
        } else {
            // Check if it's a free block, otherwise return error about missing block
            //
            // BS: Free blocks do not have IDs. Because of this, you never need to search the free_blocks
            // Vector.
            // if let Some(block) = self.free_blocks.iter().find(|&b| b.get_id() == id) {
            Err(format!("Nothing at [{}]", id))
            /* } else {
                Err(format!("Nothing at [{}]", id))
            } */
        }
    }

    pub fn dump(&self) {
        let mut all_blocks = vec![];

        // Collect information from allocated blocks
        for (&id, block) in &self.allocated_blocks {
            let start = block.get_start();
            let size = block.get_size();
            let end = start + size - 1;
            let data = block.get_data_size();
            all_blocks.push(format!("0x{:04X} - 0x{:04X}: ALLOCATED (ID: {}) (Size: {} bytes) Data: {}", start, end, id, size, data));
        }

        // Collect information from free blocks
        for block in &self.free_blocks {
            let start = block.get_start();
            let size = block.get_size();
            let end = start + size - 1;
            all_blocks.push(format!("0x{:04X} - 0x{:04X}: FREE (Size: {} bytes)", start, end, size));
        }

        // Sorting all blocks by start address for organized output
        all_blocks.sort();
        for block in all_blocks {
            println!("{}", block);
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
                if parts.len() > 1 {
                    let id_str = parts[1]; // Capture the ID as a string
                    match id_str.parse::<usize>() {
                        Ok(id) => {
                            match self.read(id) {
                                Ok(block) => {
                                    println!(
                                        "READ data: ID: {}, Start Address: {}, End Address: {}, Size: {}, Data Length: {}",
                                        block.get_id(),
                                        block.get_start(),
                                        block.get_end(),
                                        block.get_size(),
                                        block.get_data_size()
                                    );
                                },
                                Err(e) => {
                                    println!("Error reading block: {}", e);
                                }
                            }
                        },
                        Err(_) => println!("Invalid ID format provided.")
                    }
                } else {
                    println!("No ID provided for READ command.");
                }
            },
            "DUMP" => {
                self.dump();
            },
            _ => println!("Unknown command"),
        }
    }
}
