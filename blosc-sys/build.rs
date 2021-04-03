// vim: tw=80
use std::env;

fn main() {
    let prefer_static = env::var_os("BLOSC_STATIC").is_some();
    pkg_config::Config::new()
        .statik(prefer_static)
        .probe("blosc")
        .unwrap();
}
