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

use crate::config::default;


// Static-like method to get block size without an instance (temporary file usage)
pub fn gen_block_size() -> usize {
    // Create a temporary file in the system's temporary directory
    let temp_file_path = "/"; //temp_dir().join("temp_block_file");

    // Try to get block size using the temporary file, fallback to default
    let block_size = get_block_size(&temp_file_path).unwrap_or(default::BLOCK_SIZE);

    // Clean up: delete the temporary file
    //let _ = std::fs::remove_file(&temp_file_path);

    block_size
}


// Helper function to dynamically determine the block size for a given file path
pub fn get_block_size<P: AsRef<Path>>(file_path: P) -> Option<usize> {
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

