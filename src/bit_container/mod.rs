use std::{error::Error, fmt::Display, fs::File, io::Write};

pub struct BitContainer {
    byte_buffer: Vec<u8>,
    current_byte: u8,
    cutter: u8,
}

impl BitContainer {
    pub fn new() -> Self {
        Self {
            byte_buffer: Vec::new(),
            current_byte: 0,
            cutter: 0,
        }
    }

    pub fn add_code(&mut self, code: &Vec<bool>) {
        for bit in code {
            self.add_bit(*bit);
        }
    }

    pub fn add_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << self.cutter;
        }

        self.cutter += 1;
        if self.cutter > 7 {
            self.flush()
        }
    }

    /// writes stored bits into passed file and clears its buffers
    pub fn flush_to_file(&mut self, file: &mut File) -> Result<(), Box<dyn Error>> {
        self.flush();

        let result = Ok(file.write_all(self.get_all_bytes())?);

        self.byte_buffer.clear();
        self.current_byte = 0;
        self.cutter = 0;

        result
    }

    pub fn finish(&mut self) {
        self.flush()
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

    pub fn get_bits(&self) -> Vec<bool> {
        let mut bits = vec![];
        for byte in &self.byte_buffer {
            for i in 0..8 {
                bits.push((byte & (1 << i)) != 0);
            }
        }

        bits
    }

    // potentially weak place!
    fn flush(&mut self) {
        if self.current_byte > 0 || self.byte_buffer.is_empty() {
            self.byte_buffer.push(self.current_byte);
        }
        self.current_byte = 0;
        self.cutter = 0;
    }
}

impl From<Vec<u8>> for BitContainer {
    fn from(value: Vec<u8>) -> Self {
        let mut container = Self::new();

        for bit in value {
            container.add_bit(bit != 0);
        }

        container
    }
}

impl From<Vec<bool>> for BitContainer {
    fn from(value: Vec<bool>) -> Self {
        let mut container = Self::new();

        for bit in value {
            container.add_bit(bit);
        }

        container
    }
}

impl Display for BitContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();

        let mut byte_cutter = 0;
        for bit in self.get_bits() {
            repr.push(if bit { '1' } else { '0' });

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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::BitContainer;

    #[test]
    fn test_bit_container() {
        let mut cont = BitContainer::new();

        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(false);

        assert_eq!(cont.get_bytes(1), [59]);

        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);

        assert_eq!(cont.get_bytes(4), [59, 170]);

        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(true);

        assert_eq!(cont.get_bytes(4), [59, 170, 235]);

        cont.add_bit(true);
        cont.add_bit(true);

        assert_eq!(cont.get_bytes(4), [59, 170, 235]);

        cont.finish();

        assert_eq!(cont.get_bytes(4), [59, 170, 235, 3]);
    }

    #[test]
    fn test_flush() {
        let mut cont = BitContainer::new();

        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(false);

        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);

        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(false);
        cont.add_bit(true);
        cont.add_bit(true);
        cont.add_bit(true);

        cont.add_bit(true);
        cont.add_bit(true);

        let mut file = File::create("/home/nazar/prg/rust/labs/tik/samples/test_bits").unwrap();
        cont.flush_to_file(&mut file).unwrap();
    }
}
