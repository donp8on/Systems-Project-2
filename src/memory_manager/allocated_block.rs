// BS: Consider adding implementing Display for AllocatedBlock. This will make printing
// AllocatedBlocks easier becasue you'll be able to do: `println!("{}", allocated_block)`. However,
// you'll still need to print the data contained by the AllocatedBlock.
//
// Consider creating a trait called `MemoryBlock` (or something similar) to be implemented
// by AllocatedBlock and FreeBlock. This will be useful when implementing MemoryManager.dump().

/// AllocatedBlock is a handle to allocated memory by the memory manager. In this struct, size
/// is the amount of blocks contained by the block while data_size corresponds to the amount of
/// bytes occupied by the contained data. Additionally, each allocated block has an ID.
/// 
/// AP: I added the MemoryBlock trait to AllocatedBlock. This trait has three methods: get_start(),
/// get_size(), and get_id(). This trait is implemented by AllocatedBlock.

use std::fmt;

#[derive(Clone, Debug)]
pub struct AllocatedBlock {
    pub start: usize,
    pub size: usize,
    pub id: usize,
    pub data_size: usize,
    pub data: Vec<u8>,
}

impl AllocatedBlock {
    /// Creates a new allocated block with the specified start position, size, ID, and data size.
    pub(crate) fn new(start: usize, size: usize, id: usize, data_size: usize) -> Self {
        AllocatedBlock {
            start,
            size,
            id,
            data_size,
            data: vec![0; data_size],
        }
    }
    
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    /// Returns the size field of the allocated block.
    pub(crate) fn get_size(&self) -> usize {
        self.size
    }

    /// Returns the start field of the allocated block.
    pub(crate) fn get_start(&self) -> usize {
        self.start
    }

    /// Returns the id field of the allocated block.
    pub(crate) fn get_id(&self) -> usize {
        self.id
    }

    /// Returns the data_size field of the allocated block.
    pub(crate) fn get_data_size(&self) -> usize {
        self.data_size
    }

    pub(crate) fn set_data_size(&mut self, data_size: usize) {
        self.data_size = data_size;
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

// Implement Display for AllocatedBlock
impl fmt::Display for AllocatedBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AllocatedBlock: start={}, size={}, id={}, data_size={}",
            self.start, self.size, self.id, self.data_size
        )
    }
}

// Trait MemoryBlock to be implemented by AllocatedBlock
pub trait MemoryBlock {
    fn get_start(&self) -> usize;
    fn get_size(&self) -> usize;
    fn get_id(&self) -> usize;
}

impl MemoryBlock for AllocatedBlock {
    fn get_start(&self) -> usize {
        self.start
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_id(&self) -> usize {
        self.id    
    }
}

pub trait DataMemoryBlock: MemoryBlock {
    fn get_data_size(&self) -> usize;
    fn get_end(&self) -> usize;
}

impl DataMemoryBlock for AllocatedBlock {
    fn get_data_size(&self) -> usize {
        self.data_size
    }
    fn get_end(&self) -> usize {
        self.start + self.data_size - 1
    }
}