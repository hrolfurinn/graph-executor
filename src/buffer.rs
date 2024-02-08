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

    pub fn read(&self) -> &Vec<u8> {
        &self.storage
    }

    pub fn write(&mut self, data: &Vec<u8>) {
        self.storage = data.clone();
    }
    
    pub fn size(&self) -> usize {
        self.storage.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let buffer = Buffer::new(10);
        assert_eq!(buffer.size(), 10);
        assert_eq!(buffer.read(), &vec![0; 10]);
    }

    #[test]
    fn test_buffer_write() {
        let mut buffer = Buffer::new(10);
        buffer.write(&vec![1; 10]);
        assert_eq!(buffer.read(), &vec![1; 10]);
    }
}