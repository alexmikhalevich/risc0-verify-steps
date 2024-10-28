use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Log(pub Vec<u8>);

pub const PAGE_SIZE: usize = 4096;

impl Log {
    pub fn read(path: String) -> Self {
        let mut file = File::open(path.clone()).expect(&format!("Could not open file {}", path));
        let file_len = file
            .metadata()
            .expect(&format!("Could not get file metadata {}", path))
            .len() as usize;
        let mut data = vec![0; file_len];
        file.read(&mut data)
            .expect(&format!("Could not read page data from {}", path));
        Self(data)
    }
}
