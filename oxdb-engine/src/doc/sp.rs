use std::cmp::max;

use crate::config::default;





#[allow(unused)]

struct DelStore {
    
}
struct SlottedPage {
    page_len: u16,            // page lene which is n
    total_slots: u32,         // Number of slots
    slot_positions: Vec<u32>, // Positions of byte data in the page
    data: Vec<u8>,            // Actual byte data stored in the page
    page_size: u32,
}
#[allow(unused)]
impl SlottedPage {
    pub fn new(page_len: u16) -> Self {
        Self {
            page_len,
            total_slots: 0,
            slot_positions: Vec::new(),
            data: Vec::new(),
            page_size: page_len as u32 * default::BLOCK_SIZE as u32,
        }
    }

    fn can_fit(&self, bytedata_size: usize, new_slots: usize) -> bool {
        // Size required for the header :
        // (2 byte pagelen + 2 byte blocksize +2 * total_slots bytes) + slot position array
        let header_size = 1 + 2 + 2 * (self.total_slots as usize + new_slots);
        let total_size = header_size + self.data.len() + bytedata_size;
        total_size <= self.page_size as usize
    }

    fn add_data(&mut self, bytedata: Vec<u8>) {
        // Add the position of the new data (relative to the start of the data section)
        let new_data_position = (self.page_size as usize - self.data.len() - bytedata.len()) as u32;
        self.slot_positions.push(new_data_position);

        // Add the data itself
        self.data.extend(bytedata);

        // Increment the slot count
        self.total_slots += 1;
    }
}

#[allow(unused)]
pub struct SlottedPageManager {
    pages: Vec<SlottedPage>,
}
#[allow(unused)]
impl SlottedPageManager {
    pub fn new() -> Self {
        Self { pages: Vec::new() }
    }

    pub fn push(&mut self, bytedataarray: Vec<Vec<u8>>) -> Vec<(Vec<usize>, Vec<u8>)> {
        let mut page_len = max(
            ((((bytedataarray[0].len() + 2 + 2 + 4 + 4) as f64 / default::BLOCK_SIZE as f64).ceil()) as u16),
            default::MAX_PAGE_LEN,
        );
        let mut results: Vec<(Vec<usize>, Vec<u8>)> = Vec::new();
        let mut current_page = SlottedPage::new(page_len as u16);

        let mut data_indices: Vec<usize> = Vec::new();
        for (i, bytedata) in bytedataarray.into_iter().enumerate() {
            if current_page.can_fit(bytedata.len(), 1) {
                // Add the byte data to the current page if it fits
                current_page.add_data(bytedata);
                data_indices.push(i);
            } else {
                // If the current page is full, store it and create a new page
                let page_bytes = self.create_page_bytes(&current_page);
                results.push((data_indices.clone(), page_bytes));

                // Start a new page
                let mut page_len = max(
                    ((((bytedata.len() + 2 + 2 + 4 + 4) as f64 / default::BLOCK_SIZE as f64).ceil()) as u16),
                    default::MAX_PAGE_LEN,
                );
                current_page = SlottedPage::new(page_len);
                data_indices.clear();

                // Add the byte data to the new page
                current_page.add_data(bytedata);
                data_indices.push(i);
            }
        }

        // Push the last page after the loop
        let page_bytes = self.create_page_bytes(&current_page);
        results.push((data_indices, page_bytes));

        results
    }

    fn create_page_bytes(&self, page: &SlottedPage) -> Vec<u8> {
        let mut result = Vec::new();

        // Add pagelen blocksize 
        result.extend(&(page.page_len).to_be_bytes());
        result.extend(&default::BLOCK_SIZE.to_be_bytes());

        // Add the total number of slots (2 bytes)
        result.extend(&(page.total_slots).to_be_bytes());

        // Add the slot positions (2 bytes each)
        for &position in &page.slot_positions {
            result.extend(&position.to_be_bytes());
        }

        // Add padding if necessary (align slot position data and the actual data)
        let slot_section_len = 2 + 2 + 4 + 4 * page.total_slots as usize;
        println!(
            "l:{} , {}, {}",
            page.page_size as usize,
            slot_section_len,
            page.data.len()
        );
        let padding_len = page.page_size as usize - slot_section_len - page.data.len();

        if padding_len > 0 {
            result.extend(vec![0u8; padding_len]);
        }

        // Finally, add the actual byte data
        result.extend(&page.data);

        result
    }
}

#[test]
fn main() {
    let mut manager = SlottedPageManager::new();
    let sam = vec![0u8; 1 * 10];
    println!("{:?}", sam);
    let bytedataarray = vec![
        vec![0u8; 20 * 1024], // 20 KB data
        vec![0u8; 50 * 1024], // 50 KB data
        vec![0u8; 10 * 1024], // 10 KB data
        vec![0u8; 10 * 1024], // 10 KB data
        vec![0u8; 80 * 1024], // 80 KB data
        vec![0u8; 1 * 1024],
        vec![0u8; 2 * 1024],
        vec![0u8; 3 * 1024],
    ];

    let result = manager.push(bytedataarray);

    for (indices, page) in result {
        println!("Data indices: {:?}", indices);
        println!("Slotted page: {:?}", &page[1..25]);
    }
}
