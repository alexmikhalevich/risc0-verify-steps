use std::env;

fn main() {
    let zkarch_path = env::var("ZKARCH_PATH").expect("ZKARCH_PATH not set");
    println!("cargo:rustc-link-search=native={}", zkarch_path);
    println!("cargo:rustc-link-lib=static=zkarch-replay-steps");
}
