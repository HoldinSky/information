use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::os::unix::fs::FileExt;

use crate::constants::DEFAULT_BUFFER_SIZE;

pub struct FileReader {
    current_position: usize,
    buffer: [u8; DEFAULT_BUFFER_SIZE],
}

#[allow(dead_code)]
impl FileReader {
    pub fn new() -> Self {
        Self {
            current_position: 0,
            buffer: [0; DEFAULT_BUFFER_SIZE],
        }
    }

    pub fn read_entire_file(&mut self, file: &mut File) -> Result<String, Error> {
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => Ok(buf),
            Err(err) => Err(Error::new(ErrorKind::Other, err)),
        }
    }

    pub fn read_file_in_chunks<F>(
        &mut self,
        file: &File,
        start_offset: Option<usize>,
        mut action_on_chunk: F,
    ) -> Result<(), Error>
    where
        F: FnMut(&[u8], usize) -> Result<(), Error>,
    {
        self.current_position = match start_offset {
            Some(off) => off,
            None => 0,
        };

        loop {
            let bytes_read = match file.read_at(&mut self.buffer, self.current_position as u64) {
                Ok(size) => size,
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, err));
                }
            };

            self.current_position += bytes_read;

            action_on_chunk(&self.buffer, bytes_read)?;

            if bytes_read < self.buffer.len() {
                break Ok(());
            }
        }
    }
}
