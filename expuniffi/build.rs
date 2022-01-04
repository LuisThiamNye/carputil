use std::env;

fn main() {
  let crate_dir =
    env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined");

  cbindgen::generate(&crate_dir)
    .unwrap()
    // .write_to_file(out_dir.join("libruststd.h"));
    .write_to_file("../target/librustcrates.h");
}
