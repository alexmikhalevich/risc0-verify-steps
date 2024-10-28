use std::os::raw::{c_char, c_ulong};
use tiny_keccak::{Keccak, Hasher};

#[no_mangle]
pub extern "C" fn interop_merkle_tree_hash(data: *const c_char, size: c_ulong, hash: *mut c_char) {
    let mut hasher = Keccak::v256();
    if size > 32 {
        unsafe {
            let half_size = size / 2;
            let left_hash = [0u8; 32];
            interop_merkle_tree_hash(data, half_size, left_hash.as_ptr() as *mut c_char);
            let right_hash = [0u8; 32];
            interop_merkle_tree_hash(
                data.add(half_size as usize) as *const c_char,
                half_size,
                right_hash.as_ptr() as *mut c_char
            );
            hasher.update(left_hash.as_ref());
            hasher.update(right_hash.as_ref());
            let mut result_bytes = [0u8; 32];
            hasher.finalize(&mut result_bytes);
            std::ptr::copy(result_bytes.as_ptr(), hash as *mut u8, 32);
        }
    } else{
        let mut hasher = Keccak::v256();
        hasher.update(unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) });
        let mut result_bytes = [0u8; 32];
        hasher.finalize(&mut result_bytes);
        unsafe {
            std::ptr::copy(result_bytes.as_ptr(), hash as *mut u8, 32);
        }
    }
}

#[no_mangle]
pub extern "C" fn interop_concat_hash(left: *const c_char, right: *const c_char, result: *mut c_char) {
    let mut hasher = Keccak::v256();
    hasher.update(unsafe { std::slice::from_raw_parts(left as *const u8, 32) });
    hasher.update(unsafe { std::slice::from_raw_parts(right as *const u8, 32) });
    let mut result_bytes = [0u8; 32];
    hasher.finalize(&mut result_bytes);
    unsafe {
        std::ptr::copy(result_bytes.as_ptr(), result as *mut u8, 32);
    }
}
