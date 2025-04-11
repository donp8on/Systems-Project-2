use systems_project::memory_manager::MemoryManager;

#[test]
fn test_insert_and_read() {
    let mut mm = MemoryManager::new();
    let data = b"Hello";
    let id = mm.insert(data.len()).unwrap();
    mm.set(id, data).unwrap();

    let read_output = mm.read_formatted(id).unwrap();
    assert!(read_output.contains("Hello"));
}

#[test]
fn test_update_data_same_block() {
    let mut mm = MemoryManager::new();
    let id = mm.insert(10).unwrap();
    mm.set(id, b"HELLO").unwrap();
    mm.update(id, b"BYE").unwrap();

    let result = mm.read_formatted(id).unwrap();
    assert!(result.contains("BYE"));
}

#[test]
fn test_update_requires_reallocation() {
    let mut mm = MemoryManager::new();
    let id = mm.insert(4).unwrap();
    mm.set(id, b"1234").unwrap();
    mm.update(id, b"12345678").unwrap();

    assert!(mm.read(id).is_err());
}

#[test]
fn test_delete_and_merge() {
    let mut mm = MemoryManager::new();
    let id1 = mm.insert(8).unwrap();
    let id2 = mm.insert(8).unwrap();
    mm.delete(id1).unwrap();
    mm.delete(id2).unwrap();
    mm.merge_free_blocks();

    // Just make sure we can call dump without errors
    mm.dump();
}

#[test]
fn test_insert_exact_fit() {
    let mut manager = MemoryManager::new();
    let block_size = 1024;
    let id = manager.insert(block_size).unwrap();
    let data = vec![1u8; block_size];
    assert_eq!(manager.set(id, &data), Ok(()));
}

#[test]
fn test_insert_multiple_blocks() {
    let mut manager = MemoryManager::new();
    let id1 = manager.insert(500).unwrap();
    manager.set(id1, &vec![b'1'; 500]).unwrap();

    let id2 = manager.insert(1000).unwrap();
    manager.set(id2, &vec![b'2'; 1000]).unwrap();

    let id3 = manager.insert(200).unwrap();
    manager.set(id3, &vec![b'3'; 200]).unwrap();

    let out1 = manager.read_formatted(id1).unwrap();
    let out2 = manager.read_formatted(id2).unwrap();
    let out3 = manager.read_formatted(id3).unwrap();

    assert!(out1.contains("111"));
    assert!(out2.contains("222"));
    assert!(out3.contains("333"));
}


#[test]
fn test_insert_zero_size() {
    let mut mm = MemoryManager::new();
    let result = mm.insert(0);
    assert!(result.is_err(), "Inserting zero size should return an error");
}

#[test]
fn test_read_nonexistent_block() {
    let mm = MemoryManager::new();
    let result = mm.read(999);
    assert!(result.is_err(), "Reading a nonexistent block should return an error");
}

#[test]
fn test_update_nonexistent_block() {
    let mut mm = MemoryManager::new();
    let result = mm.update(42, b"new data");
    assert!(result.is_err(), "Updating a nonexistent block should return an error");
}

#[test]
fn test_delete_nonexistent_block() {
    let mut mm = MemoryManager::new();
    let result = mm.delete(42);
    assert!(result.is_err(), "Deleting a nonexistent block should return an error");
}

#[test]
fn test_memory_full() {
    let mut mm = MemoryManager::new();
    let mut ids = Vec::new();

    // Try to fill the entire memory
    let mut total = 0;
    while let Ok(id) = mm.insert(1024) {
        ids.push(id);
        total += 1024;
        if total > 65536 {
            break;
        }
    }

    // Ensure further allocations fail
    assert!(mm.insert(1024).is_err(), "Should fail once memory is full");
}

