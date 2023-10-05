use std::fs::File;
use std::io::Read;
use std::os::unix::fs::FileExt;

const DEFAULT_BUFFER_SIZE: usize = 2_097_152;

pub struct FileReader {
    current_position: u64,
    buffer: [u8; DEFAULT_BUFFER_SIZE],
}

#[allow(dead_code)]
impl FileReader {
    pub fn new() -> Self {
        Self { current_position: 0, buffer: [0; DEFAULT_BUFFER_SIZE] }
    }

    pub fn read_entire_file(&mut self, filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = match File::open(filepath) {
            Ok(file) => file,
            Err(err) => { return Err(Box::new(err)); }
        };

        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => Ok(buf),
            Err(err) => Err(Box::new(err))
        }
    }

    pub fn read_file_in_chunks<F: FnMut(&[u8]) -> ()>(&mut self, filepath: &str, mut action_on_chunk: F) -> Result<(), Box<dyn std::error::Error>> {
        self.current_position = 0;
        let file = match File::open(filepath) {
            Ok(file) => file,
            Err(err) => { return Err(Box::new(err)); }
        };

        loop {
            let bytes_read = match file.read_at(&mut self.buffer, self.current_position) {
                Ok(size) => size,
                Err(err) => { return Err(Box::new(err)); }
            };

            self.current_position += bytes_read as u64;

            action_on_chunk(&self.buffer[..bytes_read]);

            if bytes_read < self.buffer.len() { break Ok(()); }
        }
    }
}