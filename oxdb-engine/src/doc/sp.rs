

#[allow(unused)]




const PAGE_SIZE: usize = 64 * 1024; // 64 KB

#[allow(unused)]
struct SlottedPage {
    xsid: u8,                 // Page start position of slotted array
    total_slots: u16,          // Number of slots
    slot_positions: Vec<u16>,  // Positions of byte data in the page
    data: Vec<u8>,             // Actual byte data stored in the page
}
#[allow(unused)]
impl SlottedPage {
    pub fn new(xsid: u8) -> Self {
        Self {
            xsid,
            total_slots: 0,
            slot_positions: Vec::new(),
            data: Vec::new(),
        }
    }

    fn can_fit(&self, bytedata_size: usize, new_slots: usize) -> bool {
        // Size required for the header: 4 bytes (xsid + total_slots) + slot position array
        let header_size = 4 + 2 * (self.total_slots as usize + new_slots);
        let total_size = header_size + self.data.len() + bytedata_size;
        total_size <= PAGE_SIZE
    }

    fn add_data(&mut self, bytedata: Vec<u8>) {
        // Add the position of the new data (relative to the start of the data section)
        let new_data_position = (PAGE_SIZE - self.data.len() - bytedata.len()) as u16;
        self.slot_positions.push(new_data_position);

        // Add the data itself
        self.data.extend(bytedata);

        // Increment the slot count
        self.total_slots += 1;
    }
}


#[allow(unused)]
struct SlottedPageManager {
    pages: Vec<SlottedPage>,
}
#[allow(unused)]
impl SlottedPageManager {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
        }
    }

    pub fn push(&mut self, bytedataarray: Vec<Vec<u8>>) -> Vec<(Vec<usize>, Vec<u8>)> {
        let mut page_index = 0;
        let mut results: Vec<(Vec<usize>, Vec<u8>)> = Vec::new();
        let mut current_page = SlottedPage::new(page_index as u8);

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
                page_index += 1;
                current_page = SlottedPage::new(page_index as u8);
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

        // Add the xsid (1 byte)
        result.push(page.xsid);

        // Add the total number of slots (2 bytes)
        result.extend(&(page.total_slots).to_be_bytes());

        // Add the slot positions (2 bytes each)
        for &position in &page.slot_positions {
            result.extend(&position.to_be_bytes());
        }

        // Add padding if necessary (align slot position data and the actual data)
        let slot_section_len = 4 + 2 * page.total_slots as usize;
        let padding_len = PAGE_SIZE - slot_section_len - page.data.len();
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
    let sam  = vec![0u8; 1 * 10];
    println!("{:?}",sam);
    let bytedataarray = vec![
        vec![0u8; 20 * 1024], // 20 KB data
        vec![0u8; 50 * 1024], // 50 KB data
        vec![0u8; 10 * 1024], // 10 KB data
        vec![0u8; 10 * 1024], // 10 KB data
    ];

    let result = manager.push(bytedataarray);

    for (indices, page) in result {
        println!("Data indices: {:?}", indices);
        println!("Slotted page: {:?}", page[25]);
    }
}



