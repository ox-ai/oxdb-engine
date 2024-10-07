use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::config::default;
use crate::doc::prop;

// Struct representing the block-based file handler
/// ## docblock
/// ```
/// let doc_path = "data.oxd";
/// let mut doc = DocBlock::new(doc_path)?;
/// ```
pub struct DocBlock {
    file: File,
    block_size: usize,
}


impl DocBlock {
    // Function to create or open a file and determine the block size
    pub fn new(doc_path: &str) -> io::Result<Self> {
        // Open file for both reading and writing, create if it doesn't exist
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(doc_path)?;

        // Get block size dynamically, falling back to the default if necessary
        let block_size = prop::get_block_size(doc_path).unwrap_or(default::BLOCK_SIZE);

        Ok(DocBlock { file, block_size })
    }

    /// ## Method to get the block size of doc
    /// ```
    /// doc.get_block_size()
    /// ```
    pub fn get_block_size(&self) -> usize {
        self.block_size
    }

    /// ## Write data to the doc (file) in block of pages
    /// ```
    /// // Write data at the end (append)
    /// doc.write(&encoded_data, None,None)?;
    /// ```
    /// ```
    /// // Write data to a specific block index (e.g., index 2)
    /// doc.write(&encoded_data, Some(2),Some(2))?;
    /// ```
    /// 
    pub fn write(&mut self, data: &[u8], index: Option<usize>,page_len:Option<usize>) -> io::Result<()> {
        let p_len = match page_len {
            Some(value) => value,
            None => 1,
        };
        let page_size = p_len * self.block_size;
        let mut padded_data = data.to_vec();

        // Ensure data is exactly one block in size (pad if necessary)
        if padded_data.len() < page_size {
            padded_data.resize(page_size, 0); // Pad with zeros if data is less than a block
        }

        // Determine the write position
        if let Some(idx) = index {
            // Write at the specific block index
            self.file.seek(SeekFrom::Start((idx * self.block_size) as u64))?;
        } else {
            // Append to the end of the file
            self.file.seek(SeekFrom::End(0))?;
        }

        // Write the block
        self.file.write_all(&padded_data)?;
        self.file.flush()?; // Ensure data is written to disk

        Ok(())
    }

    /// ## Read data from the file in blocks
    /// ```
    /// // Read the first block (index not given)
    /// let block = doc.read(None,None)?;
    /// ```
    /// ```
    /// // Read the block at index 2
    /// let block = doc.read(Some(2),Some(2))?;
    /// ```
    /// 
    pub fn read(&mut self, index: Option<usize>,page_len:Option<usize>) -> io::Result<Vec<u8>> {
        let block_size = self.block_size;
        let p_len = match page_len {
            Some(value) => value,
            None => 1,
        };
        let page_size = p_len * self.block_size;
        
        let mut buffer = vec![0; page_size]; // Buffer to store the block

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


#[test]
fn main() -> io::Result<()> {
    // Example usage of DocBlock

    // Use the static-like method to get the block size without an instance
    let block_size = prop::gen_block_size();
    println!("Block size (from temp file): {}", block_size);

    // instance to get the block size
    let doc_path = "temp_doc.oxd";
    let mut doc_block = DocBlock::new(doc_path)?;
    println!(
        "Block size (from instance): {}",
        doc_block.get_block_size()
    );

    // Data to write (less than 4KB, will be padded)
    let data = b"Hello, database!";

    // Write data at the end (append)
    doc_block.write(data, None,None)?;

    // Write data to a specific block index (e.g., index 2)
    doc_block.write(data, Some(2),Some(1))?;

    // Read the first block (index not given)
    let block = doc_block.read(None,None)?;
    println!("Read block: {:?}", String::from_utf8_lossy(&block));

    // Read the block at index 2
    let block = doc_block.read(Some(2),Some(1))?;
    println!(
        "Read block at index 2: {:?}",
        String::from_utf8_lossy(&block)
    );

    Ok(())
}


