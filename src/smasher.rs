pub struct SFile {     
    path: String,        
    data: Vec<u8>,       // содержимое файла в байтах
    original_size: u64,  // размер до сжатия
    
}

impl SFile {
    pub fn new(path: String, data: Vec<u8>) -> Self {
        let original_size = data.len() as u64;
        SFile {
            path,
            original_size,
            data
        }
    }

    //sets
    // gets
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn get_original_size(&self) -> u64 {
        self.original_size
    }
    
}