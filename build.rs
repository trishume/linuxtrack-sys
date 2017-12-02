extern crate cc;

use std::env;

fn main() {
    cc::Build::new().file("linuxtrack.c").warnings(false).compile("linuxtrackc");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
