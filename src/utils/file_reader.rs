use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::os::unix::fs::FileExt;

use super::constants::DEFAULT_BUFFER_SIZE;

pub struct FileReader {
    current_position: usize,
    buffer: [u8; DEFAULT_BUFFER_SIZE],
    file: File,
}

#[allow(dead_code)]
impl FileReader {
    pub fn new(file: File) -> Self {
        Self {
            current_position: 0,
            buffer: [0; DEFAULT_BUFFER_SIZE],
            file: file,
        }
    }

    pub fn read_entire_file(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        match self.file.read_to_string(&mut buf) {
            Ok(_) => Ok(buf),
            Err(err) => Err(Error::new(ErrorKind::Other, err)),
        }
    }

    pub fn read_file_in_chunks<F>(&mut self, mut action_on_chunk: F) -> Result<(), Error>
    where
        F: FnMut(&[u8], bool) -> Result<(), Error>,
    {
        let file_size = self.file.metadata().unwrap().len();

        loop {
            let bytes_read = match self
                .file
                .read_at(&mut self.buffer, self.current_position as u64)
            {
                Ok(size) => size,
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, err));
                }
            };

            self.current_position += bytes_read;
            let finished_file = self.current_position as u64 >= file_size;

            action_on_chunk(&self.buffer[..bytes_read], finished_file)?;

            if finished_file {
                break Ok(());
            }
        }
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.current_position = offset;
    }

    pub fn rewind(&mut self) {
        self.current_position = 0;
    }
}
