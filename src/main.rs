use systems_project::memory_manager::MemoryManager;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};  


/// Main function to read commands from a file and execute them
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_cmmd_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open the file");
    let reader = BufReader::new(file);

    let mut manager = MemoryManager::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        manager.execute_command(&line);
    }
}