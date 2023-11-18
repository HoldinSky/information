use std::fs::File;

pub type Probability = (u8, f64);
pub type FileInfo = (File, String);
pub type FileStats = ([u64; 256], u64);
