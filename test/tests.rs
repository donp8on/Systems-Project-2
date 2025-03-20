#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deallocate_block() {
        let mut manager = MemoryManager::new();
        let block_size = nearest_power_of_two(1024);
        let id = manager.allocate(block_size).unwrap();
        let data = vec![1u8; block_size];
        manager.store(id, &data).unwrap();

        // Deallocate the block
        assert_eq!(manager.deallocate(id), Ok(()));
        // Ensure the block is now free
        assert!(manager.allocate(block_size).is_some());
    }

    #[test]
    fn test_allocate_fragmented_memory() {
        let mut manager = MemoryManager::new();
        let id1 = manager.allocate(500).unwrap();
        manager.store(id1, &vec![1u8; 500]).unwrap();

        let id2 = manager.allocate(1000).unwrap();
        manager.store(id2, &vec![2u8; 1000]).unwrap();

        // Deallocate the first block
        manager.deallocate(id1).unwrap();

        // Allocate a smaller block that fits in the freed space
        let id3 = manager.allocate(300).unwrap();
        manager.store(id3, &vec![3u8; 300]).unwrap();

        assert_eq!(manager.memory[0..300], vec![3u8; 300][..]);
        assert_eq!(manager.memory[500..1500], vec![2u8; 1000][..]);
    }

    #[test]
    fn test_store_in_unallocated_block() {
        let mut manager = MemoryManager::new();
        let id = 42; // Arbitrary ID that hasn't been allocated
        let result = manager.store(id, b"invalid");
        assert_eq!(result, Err("Invalid block ID"));
    }

    #[test]
    fn test_allocate_zero_size() {
        let mut manager = MemoryManager::new();
        let result = manager.allocate(0);
        assert!(result.is_none());
    }

    #[test]
    fn test_allocate_exceeding_capacity() {
        let mut manager = MemoryManager::new();
        let total_capacity = manager.total_capacity();
        let result = manager.allocate(total_capacity + 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_exact_fit() {
        let mut manager = MemoryManager::new();
        let block_size = nearest_power_of_two(1024);
        let result = manager.allocate(block_size);
        assert!(result.is_some());
        let id = result.unwrap();
        // Create data that exactly fits the block
        let data = vec![1u8; block_size];
        assert_eq!(manager.store(id, &data), Ok(()));
    }

    #[test]
    fn test_insert_too_large() {
        let mut manager = MemoryManager::new();
        // Attempt to allocate more memory than is available
        let result = manager.allocate(70000);
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_multiple_blocks() {
        let mut manager = MemoryManager::new();
        let first_id = manager.allocate(500).unwrap();
        manager.store(first_id, &vec![1u8; 500]);

        let second_id = manager.allocate(1000).unwrap();
        manager.store(second_id, &vec![2u8; 1000]);

        let third_id = manager.allocate(200).unwrap();
        manager.store(third_id, &vec![3u8; 200]);

        // Check if each block contains the correct data and does not overlap
        assert_eq!(manager.memory[0..500], vec![1u8; 500][..]);
        assert_eq!(manager.memory[500..1500], vec![2u8; 1000][..]);
        assert_eq!(manager.memory[1500..1700], vec![3u8; 200][..]);
    }
}
