use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
// use std::env::temp_dir;  // For generating temporary files

// Cross-platform imports
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use std::ptr::null_mut;
#[cfg(windows)]
use winapi::ctypes::c_char;
#[cfg(windows)]
use winapi::um::fileapi::GetDiskFreeSpaceA;

// Constants
const DEFAULT_BLOCK_SIZE: usize = 4096; // 4KB fallback block size

// Struct representing the block-based file handler
pub struct BlockFile {
    file: File,
    block_size: usize,
}

impl BlockFile {
    // Function to create or open a file and determine the block size
    pub fn new(file_path: &str) -> io::Result<Self> {
        // Open file for both reading and writing, create if it doesn't exist
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        // Get block size dynamically, falling back to the default if necessary
        let block_size = Self::get_block_size_from_path(file_path).unwrap_or(DEFAULT_BLOCK_SIZE);

        Ok(BlockFile { file, block_size })
    }

    // Method to get the block size either from the instance or using a temp file
    pub fn get_block_size(&self) -> usize {
        self.block_size
    }

    // Static-like method to get block size without an instance (temporary file usage)
    pub fn gen_block_size() -> usize {
        // Create a temporary file in the system's temporary directory
        let temp_file_path = "/"; //temp_dir().join("temp_block_file");

        // Try to get block size using the temporary file, fallback to default
        let block_size =
            Self::get_block_size_from_path(&temp_file_path).unwrap_or(DEFAULT_BLOCK_SIZE);

        // Clean up: delete the temporary file
        //let _ = std::fs::remove_file(&temp_file_path);

        block_size
    }

    // Helper function to dynamically determine the block size for a given file path
    fn get_block_size_from_path<P: AsRef<Path>>(file_path: P) -> Option<usize> {
        // Try to determine the block size depending on the OS
        #[cfg(unix)]
        {
            // On Unix systems, use the metadata's block size (blksize)
            if let Ok(metadata) = std::fs::metadata(file_path) {
                return Some(metadata.blksize() as usize);
            }
        }

        #[cfg(windows)]
        {
            // On Windows, use the GetDiskFreeSpace API to get the block size
            unsafe {
                let mut sectors_per_cluster: u32 = 0;
                let mut bytes_per_sector: u32 = 0;
                let mut number_of_free_clusters: u32 = 0;
                let mut total_number_of_clusters: u32 = 0;

                let root_path: *const c_char = null_mut(); // Pass null for root path
                if GetDiskFreeSpaceA(
                    root_path,
                    &mut sectors_per_cluster,
                    &mut bytes_per_sector,
                    &mut number_of_free_clusters,
                    &mut total_number_of_clusters,
                ) != 0
                {
                    return Some((sectors_per_cluster * bytes_per_sector) as usize);
                }
            }
        }

        // If no block size could be determined, return None
        None
    }

    // Write data to the file in blocks
    pub fn write_block(&mut self, data: &[u8], index: Option<usize>) -> io::Result<()> {
        let block_size = self.block_size;
        let mut padded_data = data.to_vec();

        // Ensure data is exactly one block in size (pad if necessary)
        if padded_data.len() < block_size {
            padded_data.resize(block_size, 0); // Pad with zeros if data is less than a block
        }

        // Determine the write position
        if let Some(idx) = index {
            // Write at the specific block index
            self.file.seek(SeekFrom::Start((idx * block_size) as u64))?;
        } else {
            // Append to the end of the file
            self.file.seek(SeekFrom::End(0))?;
        }

        // Write the block
        self.file.write_all(&padded_data)?;
        self.file.flush()?; // Ensure data is written to disk

        Ok(())
    }

    // Read data from the file in blocks
    pub fn read_block(&mut self, index: Option<usize>) -> io::Result<Vec<u8>> {
        let block_size = self.block_size;
        let mut buffer = vec![0; block_size]; // Buffer to store the block

        // Determine the read position
        if let Some(idx) = index {
            // Read at the specific block index
            self.file.seek(SeekFrom::Start((idx * block_size) as u64))?;
        } else {
            // Read from the beginning of the file
            self.file.seek(SeekFrom::Start(0))?;
        }

        // Read the block
        self.file.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}

// Unit tests for BlockFile
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}

// fn main() -> io::Result<()> {
//     // Example usage of BlockFile

//     // 1. Use the static-like method to get the block size without an instance
//     let block_size = BlockFile::gen_block_size();
//     println!("Block size (from temp file): {}", block_size);

//     // 2. Create an instance and then use the instance to get the block size
//     let file_path = "db_file.dat";
//     let mut block_file = BlockFile::new(file_path)?;
//     println!("Block size (from instance): {}", block_file.get_block_size());

//     // Data to write (less than 4KB, will be padded)
//     let data = b"Hello, database!";

//     // Write data at the end (append)
//     block_file.write_block(data, None)?;

//     // Write data to a specific block index (e.g., index 2)
//     block_file.write_block(data, Some(2))?;

//     // Read the first block (index not given)
//     let block = block_file.read_block(None)?;
//     println!("Read block: {:?}", String::from_utf8_lossy(&block));

//     // Read the block at index 2
//     let block = block_file.read_block(Some(2))?;
//     println!("Read block at index 2: {:?}", String::from_utf8_lossy(&block));

//     Ok(())
// }
