// BS: Consider adding implementing Display for AllocatedBlock. This will make printing
// AllocatedBlocks easier becasue you'll be able to do: `println!("{}", allocated_block)`. However,
// you'll still need to print the data contained by the AllocatedBlock.
//
// Consider creating a trait called `MemoryBlock` (or something similar) to be implemented
// by AllocatedBlock and FreeBlock. This will be useful when implementing MemoryManager.dump().

/// AllocatedBlock is a handle to allocated memory by the memory manager. In this struct, size
/// is the amount of blocks contained by the block while data_size corresponds to the amount of
/// bytes occupied by the contained data. Additionally, each allocated block has an ID.

use std::fmt;

#[derive(Clone, Debug)]
pub struct AllocatedBlock {
    start: usize,
    size: usize,
    id: usize,
    data_size: usize,
}

impl AllocatedBlock {
    /// Creates a new allocated block with the specified start position, size, ID, and data size.
    pub(crate) fn new(start: usize, size: usize, id: usize, data_size: usize) -> Self {
        AllocatedBlock {
            start,
            size,
            id,
            data_size,
        }
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