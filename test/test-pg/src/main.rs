// tests/test.rs

// Import BlockFile from the doc::op module
use oxdb_engine::doc::op::BlockFile;
use std::io::{self};


fn test_blockfile() -> io::Result<()> {
    // Use BlockFile's static method to get block size
    let block_size = BlockFile::gen_block_size();
    println!("Block size: {}", block_size);

    // Example path to a test file
    let file_path = "test_db_file.oxdbb";

    // Create a new BlockFile instance
    let mut block_file = BlockFile::new(file_path)?;

    // Check the block size from the instance
    assert_eq!(block_file.get_block_size(), block_size);

    // Write and read operations for testing
    let data = b"Test data";

    // Write to the file
    block_file.write_block(data, None)?;

    // Read from the file
    let block = block_file.read_block(None)?;
    // Since we know the size of `data` is 9 bytes (Test data), we only compare the first 9 bytes
    let expected_data = [b'T', b'e', b's', b't', b' ', b'd', b'a', b't', b'a'];
    assert_eq!(&block[..data.len()], &expected_data);

    // Optionally, you can check that the rest of the block is padded with zeroes
    assert!(block[data.len()..].iter().all(|&x| x == 0));
    let _ = std::fs::remove_file(&file_path);
    Ok(())
}


fn main() {
    println!("Hello, world!");
    test_blockfile().unwrap(); // Will panic on error

    
}
