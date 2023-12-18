use std::{error::Error, fmt::Display, fs::File, io::Write};

pub struct BitMap {
    byte_buffer: Vec<u8>,
    current_byte: u8,
    current_bit: u8,
}

impl BitMap {
    pub fn new() -> Self {
        Self {
            byte_buffer: Vec::new(),
            current_byte: 0,
            current_bit: 0,
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.byte_buffer.push(byte);
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.add_byte(byte.clone());
        }
    }

    pub fn add_bit_sequence(&mut self, code: &Vec<u8>) {
        for bit in code {
            self.add_bit(*bit);
        }
    }

    pub fn add_code(&mut self, code: &str) {
        for ch in code.chars() {
            self.add_bit(if ch == '1' { 1 } else { 0 });
        }
    }

    pub fn add_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.current_byte |= 1 << self.current_bit;
        }
        self.current_bit += 1;

        // when the byte is filled
        if self.current_bit > 7 {
            self.flush();
        }
    }

    /// writes stored bits into passed file and clears its buffers
    pub fn flush_to_file(&mut self, file: &mut File) -> Result<(), Box<dyn Error>> {
        self.flush();

        let result = Ok(file.write_all(self.get_all_bytes())?);

        self.clear();

        result
    }

    /// writes all complete bytes in its buffer to file. Last incomplete stays to continue filling
    pub fn flush_filled_to_file(&mut self, file: &mut File) -> Result<(), Box<dyn Error>> {
        let result = Ok(file.write_all(self.get_all_bytes())?);

        self.byte_buffer.clear();

        result
    }

    pub fn get_all_bytes(&self) -> &[u8] {
        &self.byte_buffer[..]
    }

    pub fn bytes_reserved(&self) -> usize {
        self.byte_buffer.len()
    }

    pub fn get_bytes(&self, count: usize) -> &[u8] {
        if count > self.bytes_reserved() {
            self.get_all_bytes()
        } else {
            &self.byte_buffer[..count]
        }
    }

    /// returns array of bits (1 0)
    pub fn get_all_bits(&self) -> Vec<u8> {
        let mut bits = vec![];

        for byte in &self.byte_buffer {
            for i in 0..8 {
                bits.push(if byte & (1 << i) == 0 { 0 } else { 1 });
            }
        }

        bits
    }

    pub fn get_bits(&self, count: usize) -> Vec<u8> {
        let mut bits = vec![];
        let mut count = count;

        for byte in &self.byte_buffer {
            for i in 0..8 {
                bits.push(if byte & (1 << i) == 0 { 0 } else { 1 });
                count -= 1;

                if count == 0 {
                    return bits;
                }
            }
        }

        bits
    }

    pub fn clear(&mut self) {
        self.byte_buffer.clear();
        self.current_byte = 0;
        self.current_bit = 0;
    }

    fn flush(&mut self) {
        if self.current_bit != 0 {
            self.byte_buffer.push(self.current_byte);
        }
        self.current_byte = 0;
        self.current_bit = 0;
    }
}

impl From<Vec<u8>> for BitMap {
    fn from(value: Vec<u8>) -> Self {
        let mut container = Self::new();

        for bit in value {
            container.add_bit(bit);
        }

        container
    }
}

impl From<Vec<bool>> for BitMap {
    fn from(value: Vec<bool>) -> Self {
        let mut container = Self::new();

        for bit in value {
            container.add_bit(if bit { 1 } else { 0 });
        }

        container
    }
}

impl Display for BitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();

        let mut byte_cutter = 0;
        for bit in self.get_all_bits() {
            repr.push(if bit == 1 { '1' } else { '0' });

            byte_cutter += 1;

            if byte_cutter % 4 == 0 && byte_cutter < 8 {
                repr.push('\'');
            } else if byte_cutter > 7 {
                repr.push(' ');
                byte_cutter = 0;
            }
        }

        f.pad(if repr.is_empty() { "0" } else { &repr })
    }
}
