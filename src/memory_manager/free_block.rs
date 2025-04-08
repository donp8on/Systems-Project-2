// BS: Consider adding implementing Display for FreeBlock. This will make printing FreeBlocks
// easier becasue you'll be able to do: `println!("{}", free_block)`
//
// Consider creating a trait called `MemoryBlock` (or something similar) to be
// implemented by AllocatedBlock and FreeBlock. This will be useful when implementing
// MemoryManager.dump().

use std::fmt;

/// FreeBlock struct representing a block of memory that does not contain data.
#[derive(Debug)]
pub(crate) struct FreeBlock {
    pub start: usize,
    pub size: usize,
    pub is_free: bool,
}

impl FreeBlock {
    /// Creates a new FreeBlock with the specified start position and size.
    pub(crate) fn new(start: usize, size: usize) -> Self {
        FreeBlock { start, size, is_free: true }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }
    /// Returns the size field of the free block.
    pub(crate) fn get_size(&self) -> usize {
        self.size
    }
}

// Implement Display for FreeBlock
impl fmt::Display for FreeBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FreeBlock: start={}, size={}", self.start, self.size)
    }
}

// Trait MemoryBlock to be implemented by FreeBlock
pub trait MemoryBlock {
    fn get_start(&self) -> usize;
    fn get_size(&self) -> usize;
}