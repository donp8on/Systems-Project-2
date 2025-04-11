use crate::*;
use std::collections::HashMap;

mod allocated_block;
mod free_block;
mod memory_block;

use memory_block::MemoryBlock;
use allocated_block::AllocatedBlock;
use free_block::FreeBlock;
use allocated_block::DataMemoryBlock;



/// Define the MemoryManager struct
pub struct MemoryManager {
    memory: [u8; 65535],
    free_blocks: Vec<FreeBlock>, // Using Vec to manage free blocks
    allocated_blocks: HashMap<usize, AllocatedBlock>,
    next_id: usize,
}

/// MemoryManager struct to manage memory allocation and deallocation
/// It contains a memory array, a vector of free blocks, a hashmap of allocated blocks, and an ID counter.
impl MemoryManager {
    pub fn new() -> MemoryManager {
        MemoryManager {
            memory: [0; 65535],
            free_blocks: vec![FreeBlock::new(0, 65536)], // Initial large block
            allocated_blocks: HashMap::new(),
            next_id: 0,
        }
    }

    /// Function to set data in a memory block
    /// This function will check if the block ID exists and if the data fits in the block size
    pub fn set(&mut self, id: usize, data: &[u8]) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            if data.len() <= block.size {
                // Copy the new data into the memory starting at block.start
                self.memory[block.start..(block.start + data.len())].copy_from_slice(data);
                // Update the actual used size of data in the block
                block.data_size = data.len();
                println!("Data successfully updated in block ID: {}", id);
                Ok(())
            } else {
                Err(format!("Data size exceeds block size. Data size: {}, Block size: {}", data.len(), block.size))
            }
        } else {
            Err("Block ID not found".to_string())
        }
    }

    /// This function uses 'allocate' to get a block and then inserts data into it
    /// It returns the ID of the allocated block or an error message
    /// It will also check if the data fits in the block size and update the data size accordingly
    pub fn insert(&mut self, data_size: usize) -> Result<usize, String> {
        let required_size = data_size.next_power_of_two();
    
        // Find the first block that is large enough
        let mut best_index = None;
        for (i, block) in self.free_blocks.iter().enumerate() {
            if block.is_free && block.size >= required_size {
                best_index = Some(i);
                break;
            }
        }
    
        if let Some(index) = best_index {
            let mut block = self.free_blocks.remove(index);
    
            // Split until size matches
            while block.size > required_size {
                let half_size = block.size / 2;
                let right_block = FreeBlock::new(block.start + half_size, half_size);
                self.free_blocks.push(right_block);
                block.size = half_size; // Keep the left half
            }
    
            block.is_free = false;
            let id = self.next_id;
            self.allocated_blocks.insert(id, AllocatedBlock::new(block.start, block.size, id, data_size));
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }

    /// Function to read data from a memory block
    /// This function will check if the block ID exists and return the block details
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
            Err(format!("Nothing at [{}]", id))
        }
    }
    
    /// Function to read data from the memory manager and format it for output
    /// This function will check if the block ID exists and return the formatted string
    /// It will also include the start and end addresses, status, size, and data
    pub fn read_formatted(&self, id: usize) -> Result<String, String> {
        if let Some(block) = self.allocated_blocks.get(&id) {
            let data_slice = &self.memory[block.start..block.start + block.data_size];
            let data_string = String::from_utf8_lossy(data_slice);
            Ok(format!(
                "READ data: Start Address: 0x{:04X}, End Address: 0x{:04X}, Status: Allocated, Size: {} bytes, Data: '{}'",
                block.start, block.start + block.data_size - 1, block.data_size, data_string
            ))
        } else {
            Err(format!("Block with ID {} does not exist.", id))
        }
    }
    
    /// Function to allocate a block of memory
    /// This function will find the best fitting free block, split it if necessary, and return the ID of the allocated block
    pub fn allocate(&mut self, requested_size: usize) -> Result<usize, String> {
        let required_size = requested_size.next_power_of_two();
        let mut best_index = None;
    
        for (i, block) in self.free_blocks.iter().enumerate() {
            if block.is_free && block.size >= required_size {
                best_index = Some(i);
                break;
            }
        }
    
        if let Some(index) = best_index {
            let mut block = self.free_blocks.remove(index);
    
            while block.size > required_size {
                let half_size = block.size / 2;
                let right_block = FreeBlock::new(block.start + half_size, half_size);
                self.free_blocks.push(right_block);
                block.size = half_size;
            }
    
            block.is_free = false;
            let id = self.next_id;
            self.allocated_blocks.insert(id, AllocatedBlock::new(block.start, block.size, id, requested_size));
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }
    
    
    /// Function to delete a block by ID
    /// This function will remove the block from the allocated_blocks and add it back to the free_blocks
    /// It will also merge adjacent free blocks if necessary
    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        // Attempt to find and remove the allocated block
        if let Some(block) = self.allocated_blocks.remove(&id) {
            // Add the block back to the free_blocks list
            let new_free_block = FreeBlock::new(block.start, block.size);
            self.free_blocks.push(new_free_block);
            self.merge_free_blocks(); // Merge adjacent free blocks if possible
            Ok(())
        } else {
            Err("Block ID not found".to_string())
        }
    }

    /// Function to merge adjacent free blocks
    /// This function will iterate through the free_blocks and merge them if they are adjacent and of the same size
    /// It will also sort the free blocks by start address for consistent traversal
    pub fn merge_free_blocks(&mut self) {
        let mut changed = true;
    
        while changed {
            changed = false;
    
            // Sort free blocks by start address for consistent traversal
            self.free_blocks.sort_by_key(|block| block.start);
    
            let mut merged = Vec::new();
            let mut skip = std::collections::HashSet::new();
    
            for i in 0..self.free_blocks.len() {
                if skip.contains(&i) {
                    continue;
                }
    
                let current = &self.free_blocks[i];
                let mut merged_this_round = false;
    
                for j in (i + 1)..self.free_blocks.len() {
                    if skip.contains(&j) {
                        continue;
                    }
    
                    let next = &self.free_blocks[j];
    
                    // Only merge if sizes match and buddy condition holds
                    if current.size == next.size
                        && (current.start ^ current.size) == next.start
                    {
                        let new_start = usize::min(current.start, next.start);
                        let new_size = current.size * 2;
                        merged.push(FreeBlock::new(new_start, new_size));
                        skip.insert(i);
                        skip.insert(j);
                        changed = true;
                        merged_this_round = true;
                        break;
                    }
                }
    
                // If not merged, retain current block
                if !merged_this_round && !skip.contains(&i) {
                    merged.push(current.clone());
                }
            }
    
            self.free_blocks = merged;
        }
    }
    
    /// Function to update data in an allocated block
    /// This function will check if the new data fits in the existing block or if it needs to be reallocated
    /// If it needs to be reallocated, it will allocate a new block and copy the data over
    pub fn update(&mut self, id: usize, new_data: &[u8]) -> Result<(), String> {
        println!("Updating ID: {}, New Data: {:?}", id, String::from_utf8_lossy(new_data));
    
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            println!("Found block: start = {}, size = {}, current data size = {}", block.start, block.size, block.data_size);
    
            if new_data.len() > block.size {
                // If new data doesn't fit, reallocate
                println!("New data size exceeds current block size, reallocating...");
                let new_id = self.allocate(new_data.len())?;
                let new_block = self.allocated_blocks.get_mut(&new_id).unwrap();
    
                // Write new data to memory
                self.memory[new_block.start..new_block.start + new_data.len()]
                    .copy_from_slice(new_data);
                new_block.data_size = new_data.len();
    
                self.delete(id)?; // Free old block
                println!("Reallocated with new ID: {}", new_id);
            } else {
                // Clear existing memory region
                let block_start = block.start;
                let block_end = block_start + block.size;
                self.memory[block_start..block_end].fill(0); // Clear whole block
    
                // Write updated data
                self.memory[block_start..block_start + new_data.len()]
                    .copy_from_slice(new_data);
                block.data_size = new_data.len();
    
                println!("Data updated within existing block");
            }
    
            Ok(())
        } else {
            Err("Block ID not found".to_string())
        }
    }
    
    /// Function to dump the memory manager's state
    /// This function will print the details of allocated and free blocks in a formatted manner
    pub fn dump(&self) {
        let mut allocated = Vec::new();
        let mut free_blocks = Vec::new();
    
        // Collect allocated blocks
        for (id, block) in &self.allocated_blocks {
            let info = format!(
                "0x{:04X} - 0x{:04X}: ALLOCATED (ID: {}) (Size: {} bytes) Data: '{}'",
                block.start,
                block.start + block.size - 1,
                id,
                block.size,
                String::from_utf8_lossy(&self.memory[block.start..block.start + block.data_size])
            );
            allocated.push((block.start, info));
        }
    
        // Collect free blocks
        for block in &self.free_blocks {
            let info = format!(
                "0x{:04X} - 0x{:04X}: FREE (Size: {} bytes)",
                block.start,
                block.start + block.size - 1,
                block.size
            );
            free_blocks.push((block.size, block.start, info));
        }
    
        // Sort free blocks by size ascending, then by start address
        free_blocks.sort_by_key(|(size, start, _)| (*size, *start));
    
        // Sort allocated blocks by start address
        allocated.sort_by_key(|(start, _)| *start);
    
        println!("Memory Dump:");
        for (_, line) in allocated {
            println!("{}", line);
        }
        for (_, _, line) in free_blocks {
            println!("{}", line);
        }
    }
    
    

    /// Function to execute commands from the input file (.cmmd)
    /// This function will parse the command and call the appropriate function
    /// It will also handle the command format and print the results
    pub fn execute_command(&mut self, command: &str) {
        let command = command.to_uppercase();
        // Trim the command and remove the trailing semicolon if present
        let command = command.trim();
        let command = command.trim_end_matches(';');
    
        // Split the command into parts
        let parts: Vec<&str> = command.split_whitespace().collect();
    
        // Match the command keyword and execute accordingly
        match parts.get(0).map(|s| *s) {
            Some("INSERT") if parts.len() > 2 => {
                let size = parts[1].parse::<usize>().unwrap_or(0);
                let data = parts[2..].join(" "); // Join the remaining parts to form the data string
                if let Ok(id) = self.insert(size) {
                    if self.set(id, data.as_bytes()).is_ok() {
                        println!("INSERT success: ID = {}", id);
                    } else {
                        println!("Error storing data");
                    }
                } else {
                    println!("INSERT error: No suitable block available");
                }
            },
            Some("READ") if parts.len() > 1 => {
                let id = parts[1].parse::<usize>().unwrap_or(0);
                if let Ok(details) = self.read_formatted(id) {
                    println!("{}", details);
                } else {
                    println!("Error reading data");
                }
            },
            Some("DELETE") if parts.len() > 1 => {
                let id = parts[1].parse::<usize>().unwrap_or(0);
                if self.delete(id).is_ok() {
                    println!("DELETE success: ID = {}", id);
                } else {
                    println!("Error deleting data");
                }
            },
            Some("UPDATE") if parts.len() > 2 => {
                let id = parts[1].parse::<usize>().unwrap_or(0);
                let new_data = parts[2..].join(" "); // Ensure this captures all intended data
                if self.update(id, new_data.as_bytes()).is_ok() {
                    println!("UPDATE success: ID = {}", id);
                } else {
                    println!("Error updating data");
                }
            },
            
            Some("DUMP") => {
                self.dump();
            },
            Some("EXIT") => {
                println!("Exiting...");
                std::process::exit(0);
            },
            _ => println!("Unknown or invalid command"),
        }
    }
    
}
