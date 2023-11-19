use std::fs::File;

pub type Probability = (u8, f64); // byte -> its probability
pub type FileInfo = (File, String); // file -> its full path
pub type FileStats = ([u64; 256], u64); // dictionary -> total count of symbols
