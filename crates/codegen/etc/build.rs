extern crate codegen;

use codegen::config;
use codegen::generate_crate_exports;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=../../../src/config.h");
    config::run()?;
    generate_crate_exports()?;
    Ok(())
}
