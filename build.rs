use std::{fs::File, io::Read};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cv_data.json");
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("cv_data.rs");
    let mut file = File::open("cv_data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    std::fs::write(&path, format!("const RAW_CV_DATA: &str = r#\"{data}\"#;")).unwrap();
}
