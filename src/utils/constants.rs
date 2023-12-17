pub const DICTIONARY_END: [u8; 2] = [0, 0]; // marker for end of header of compressed file
pub const DEFAULT_BUFFER_SIZE: usize = 2_097_152; // buffer size for file reading with FileReader
pub const ARCHIVE_EXTENSION: &str = ".nk";
pub const HAMMING_CODE_LENGTH_KEY: [u8; 5] = [104, 109, 109, 99, 108]; // 104 - h; 109 - m; 99 - c; 108 ; l
