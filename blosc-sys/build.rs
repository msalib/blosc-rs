// vim: tw=80
use std::env;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    if env::var_os("BLOSC_STATIC").is_some() {
        println!("cargo:rustc-link-lib=static=blosc");
    } else {
        println!("cargo:rustc-link-lib=blosc");
    }
}
