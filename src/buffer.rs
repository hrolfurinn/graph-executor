#[derive(Debug,Clone)]
pub struct Buffer {    
    // TODO: How to have the storage be only a reference 
    // TODO: Better datatype for storage
    storage: Vec<u8>
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Self {
            storage: vec![0; size],
        }
    }

    // TODO: read
    // TODO: write
    
    pub fn size(&self) -> usize {
        self.storage.len()
    }
}