pub mod pages;

use pages::Log;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub steps: u64,
    pub start_hash: Vec<u8>,
    pub end_hash: Vec<u8>,
    pub log: Log,
}
