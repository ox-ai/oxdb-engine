

use std::io;

use oxdb_engine::doc::io::DocBlock;
use oxdb_engine::doc::prop;
use oxdb_engine::oxdbin::basebin;




fn main() -> io::Result<()> {
    
    let block_size = prop::gen_block_size();
    println!("Block size (from temp file): {}", block_size);

    // 2. Create an instance and then use the instance to get the block size
    let file_path = "data.oxd";
    let mut doc_block = DocBlock::new(file_path)?;
    println!(
        "Block size (from instance): {}",
        doc_block.get_block_size()
    );

    // // Data to write (less than 4KB, will be padded)
    // let data = b"Hello, database!";

    let data = serde_json::json!({
        "name": "record",
        "alive": 10,
        "is_admin": true,
        "scores": [85, 92, 88],
        "details": {
            "notes": "md",
            "vlist": ["1",2]
        }
    });

    // Encode
    let encoded_data = basebin::encode(&data);
    println!("Encoded data: {:?},{}", encoded_data, encoded_data.len());

    

    // Write data at the end (append)
    doc_block.write(&encoded_data, None,None)?;

    // Write data to a specific block index (e.g., index 2)
    doc_block.write(&encoded_data, Some(2),Some(2))?;

    // Read the first block (index not given)
    let block = doc_block.read(None,None)?;
    println!("Read block: {:?}", String::from_utf8_lossy(&block));

    // Read the block at index 2
    let block = doc_block.read(Some(2),Some(2))?;
    println!(
        "Read block at index 2: {:?}",
        String::from_utf8_lossy(&block)
    );

    // Decode
    // let (decoded_data, _)
    let decoded_data = basebin::decode(&block, 0);
    println!("Decoded data: {:?}", decoded_data.0);

    Ok(())
}











