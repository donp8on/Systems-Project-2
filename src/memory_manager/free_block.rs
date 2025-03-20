// BS: Consider adding implementing Display for FreeBlock. This will make printing FreeBlocks
// easier becasue you'll be able to do: `println!("{}", free_block)`
//
// Consider creating a trait called `MemoryBlock` (or something similar) to be
// implemented by AllocatedBlock and FreeBlock. This will be useful when implementing
// MemoryManager.dump().

use std::fmt;

/// FreeBlock struct representing a block of memory that does not contain data.
pub(crate) struct FreeBlock {
    start: usize,
    size: usize,
}

impl FreeBlock {
    /// Creates a new FreeBlock with the specified start position and size.
    pub(crate) fn new(start: usize, size: usize) -> Self {
        FreeBlock { start, size }
    }

    /// Returns the size field of the free block.
    pub(crate) fn get_size(&self) -> usize {
        self.size
    }

    /// Returns the start field of the free block.
    pub(crate) fn get_start(&self) -> usize {
        self.start
    }
}

// Implement Display for FreeBlock
impl fmt::Display for FreeBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FreeBlock: start={}, size={}", self.start, self.size)
    }
}