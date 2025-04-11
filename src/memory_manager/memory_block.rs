/// MemoryBlock struct to represent a block of memory in the memory manager
#[derive(Clone, Debug)]
pub struct MemoryBlock {
    pub start: usize,
    pub size: usize,
    pub is_free: bool,
    pub data: Vec<u8>,
}

/// Implement MemoryBlock struct
/// This struct represents a block of memory in the memory manager.
impl MemoryBlock {
    pub fn new(start: usize, size: usize) -> Self {
        MemoryBlock {
            start,
            size,
            is_free: true,  // Default to true, assuming blocks are free when created
            data: Vec::new(),
        }
    }
    
    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn allocate(&mut self, data: Vec<u8>) {
        self.is_free = false;
        self.data = data;
    }

    pub fn deallocate(&mut self) {
        self.is_free = true;
        self.data.clear();
    }

}
