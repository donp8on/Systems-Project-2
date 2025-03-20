use crate::*;
use allocated_block::AllocatedBlock;
use free_block::FreeBlock;
use std::collections::HashMap;

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

    // Function to allocate memory blocks using a buddy system
    //
    // BS: this doesn't seem correct, but I may be misunderstanding. First,
    // you're finding the correct size to allocate, then you're searching
    // MemoryManager.free_blocks for a block that will fit. Up until here makes
    // sense.
    //
    // After you find a block, you remove it from MemoryManager.free_blocks and then add it back?
    pub fn allocate(&mut self, requested_size: usize) -> Result<usize, String> {
        let size = nearest_power_of_two(requested_size);
        if let Some(index) = self
            .free_blocks
            .iter()
            .position(|block| block.get_size() >= size)
        {
            let block = self.free_blocks.remove(index);
            if block.get_size() > size {
                self.free_blocks.push(FreeBlock::new(
                    block.get_start() + size,
                    block.get_size() - size,
                ));
            }
            let allocated_block =
                AllocatedBlock::new(block.get_start(), size, self.next_id, requested_size);
            self.allocated_blocks.insert(self.next_id, allocated_block);
            let id = self.next_id;
            self.next_id += 1;
            Ok(id)
        } else {
            Err("No suitable block available".to_string())
        }
    }

    // Function to set data in allocated memory blocks
    //
    // BS: Remember to update AllocatedBlock.data_size in this method.
    pub fn set(&mut self, id: usize, data: &[u8]) -> Result<(), String> {
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            if data.len() <= block.get_size() {
                let end = block.get_start() + data.len();
                self.memory[block.get_start()..end].copy_from_slice(data);
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

    // Function to insert data into allocated memory blocks
    //
    // BS: same as the note on allocate(). That aside, what's the difference between
    // this and allocate()?
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
            Err(format!("Block with ID {} does not exist.", id))
            /* } else {
                Err(format!("Nothing at [{}]", id))
            } */
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
            }
            "READ" => {
                let id = parts[1].parse::<usize>().expect("Invalid ID format");
                match self.read(id) {
                    Ok(data) => {
                        // BS: self.read will return an AllocatedBlock. This will fill in most of
                        // the fields below, but it will not directly give you the data. To fill in
                        // the data, you'll need to read from MemoryManager.memory starting from
                        // data.get_start() to data.get_start() + data.get_size().
                        //
                        // Also, this is not the correct format to print a memory block.
                        println!(
                            "READ data: Start Address: {}, End Address: {}, Status: {}, Size: {}, Data Length: {}",
                            "data.0", "data.1", "data.2", "data.3", "data.4"
                        );
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            _ => println!("Unknown command"),
        }
    }
}
