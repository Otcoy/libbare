extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_no_includes()
      .with_autogen_warning("// AUTO GENERATED. DO NOT EDIT")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("include/libbare_generated.h");
}
