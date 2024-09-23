use std::fs::metadata;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

fn get_block_size<P: AsRef<Path>>(path: P) -> Option<usize> {
    let metadata = metadata(path).ok()?;
    let block_size = metadata.blksize() as usize;
    Some(block_size)
}

fn main() {
    // Use any path or mount point (e.g., "/")
    if let Some(block_size) = get_block_size("/") {
        println!("Block size: {}", block_size);
    } else {
        println!("Unable to determine block size");
    }
}
