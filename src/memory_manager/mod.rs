use crate::*;
use std::collections::HashMap;

mod allocated_block;
mod free_block;
mod memory_block;

use memory_block::MemoryBlock;
use allocated_block::AllocatedBlock;
use free_block::FreeBlock;
use allocated_block::DataMemoryBlock;



// Define the MemoryManager struct
pub struct MemoryManager {
    memory: [u8; 65536],
    free_blocks: Vec<FreeBlock>, // Using Vec to manage free blocks
    allocated_blocks: HashMap<usize, AllocatedBlock>,
    next_id: usize,
}

impl MemoryManager {
    pub fn new() -> MemoryManager {
        MemoryManager {
            memory: [0; 65536],
            free_blocks: vec![FreeBlock::new(0, 65536)], // Initial large block
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
            if data.len() <= block.size {
                // Correct the range to use the block's start and data length
                let range_end = block.start + data.len();
                if range_end <= self.memory.len() {
                    self.memory[block.start..range_end].copy_from_slice(data);
                    block.data_size = data.len(); // Make sure this field exists and is updated
                    Ok(())
                } else {
                    Err("Memory range exceeds the buffer limit".to_string())
                }
            } else {
                Err(format!(
                    "Data size exceeds block size. Data size: {}, Block size: {}",
                    data.len(),
                    block.size
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

    //function to read and return data and from allocated memory blocks
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
    
    // Function to read data from the memory manager and format it for output
    pub fn read_formatted(&self, id: usize) -> Result<String, String> {
        if let Some(block) = self.allocated_blocks.get(&id) {
            let data_slice = &self.memory[block.start..block.start + block.size];
            let data_string = String::from_utf8_lossy(data_slice);
            let formatted_string = format!(
                "READ data: Start Address: 0x{:04X}, End Address: 0x{:04X}, Status: Allocated, Size: {} bytes, Data: '{}'",
                block.start, block.start + block.size - 1, block.size, data_string
            );
            Ok(formatted_string)
        } else {
            Err(format!("Block with ID {} does not exist.", id))
        }
    }

    pub fn allocate(&mut self, requested_size: usize) -> Result<usize, String> {
        let required_size = requested_size.next_power_of_two(); // Ensuring block size is power of two may help
        for (index, block) in self.free_blocks.iter().enumerate() {
            if block.size >= required_size && block.is_free {
                let start_address = block.start;
                let remaining_size = block.size - required_size;
    
                // Update the current block or remove it if size matches exactly
                if remaining_size > 0 {
                    self.free_blocks[index] = FreeBlock::new(start_address + required_size, remaining_size);
                } else {
                    self.free_blocks.remove(index);
                }
    
                // Add the allocated block
                let allocated_block = AllocatedBlock::new(start_address, required_size, self.next_id, requested_size);
                self.allocated_blocks.insert(self.next_id, allocated_block);
                self.next_id += 1;
    
                return Ok(start_address);
            }
        }
        Err("No suitable block available".to_string())
    }


    pub fn deallocate(&mut self, address: usize) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.get_mut(&address) {
            // No need to set is_free for allocated blocks as they are not free
            block.clear_data();
            // Attempt to merge free blocks
            self.merge_free_blocks();
            Ok(())
        } else {
            Err("Block not found".to_string())
        }
    }

    pub fn merge_free_blocks(&mut self) {
        let mut to_remove = Vec::new();
        let mut to_add = Vec::new();
        let mut free_blocks: Vec<_> = self.free_blocks.iter()
            .map(|block| (block.get_start(), block.clone()))
            .collect();

        free_blocks.sort_by_key(|(k, _)| *k);

        let mut i = 0;
        while i < free_blocks.len() - 1 {
            let (current_key, current_block) = free_blocks[i];
            let (next_key, next_block) = free_blocks[i + 1];
            if current_key + current_block.size == next_key {
                // Merge blocks
                to_remove.push(current_key);
                to_remove.push(next_key);
                to_add.push(MemoryBlock {
                    start: current_key,
                    size: current_block.size + next_block.size,
                    is_free: true,
                    data: Vec::new(),
                });
                i += 1; // Skip the next block since it's merged
            }
            i += 1;
        }

        for key in to_remove {
            self.free_blocks.retain(|block| block.get_start() != key);
        }
        for block in to_add {
            self.free_blocks.push(FreeBlock::new(block.start, block.size));
        }
    }


    pub fn dump(&self) {
        for (id, block) in &self.allocated_blocks {
            let status = "ALLOCATED";
            let data_slice = &self.memory[block.start..block.start + block.data_size];
            let data_string = if block.data_size > 0 {
                String::from_utf8_lossy(data_slice)
            } else {
                "<no data>".into()
            };
            
            println!(
                "0x{:04X} - 0x{:04X}: {} (Size: {} bytes) Data: '{}'",
                block.start,
                block.start + block.size - 1,
                status,
                block.size,
                data_string
            );
        }
        for block in &self.free_blocks {
            println!(
                "0x{:04X} - 0x{:04X}: FREE (Size: {} bytes)",
                block.start,
                block.start + block.size - 1,
                block.size
            );
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
                if parts.len() < 2 {
                    println!("No ID provided for READ");
                } else {
                    let id = parts[1].parse::<usize>().expect("Invalid ID format");
                    match self.read_formatted(id) {
                        Ok(details) => println!("{}", details),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            },

            "DUMP" => {
                self.dump();
            },
            _ => println!("Unknown command"),
        }
    }
}
