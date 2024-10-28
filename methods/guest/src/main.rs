pub mod merkle;
pub mod c_utils;

use risc0_zkvm::guest::env;
use std::os::raw::{c_char, c_ulonglong};
use inputs::Inputs;

extern "C" {
    pub fn zkarch_replay_steps(
        root_hash_before: *const c_char,
        raw_log_data: *const c_char,
        raw_log_size: c_ulonglong,
        mcycle_count: c_ulonglong,
        root_hash_after: *const c_char,
    ) -> c_ulonglong;
}

fn main() {
    let input: Inputs = env::read();

    unsafe {
        // throws a runtime error if the initial root hash does not match the given pages
        // throws a runtime error if the result hash does not match `input.end_hash`
        zkarch_replay_steps(
            input.start_hash.as_ptr() as *const c_char,
            input.log.0.as_ptr() as *const c_char,
            input.log.0.len() as c_ulonglong,
            input.steps,
            input.end_hash.as_ptr() as *const c_char
        );
    }
    println!("Verification successful");

    // TODO: env::commit(hash);
}
