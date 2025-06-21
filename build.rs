use std::{fs::File, io::Read};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cv_data.json");
    println!("cargo:rerun-if-changed=avatar.mp4");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    // Generate cv_data.rs
    let cv_data_path = std::path::Path::new(&out_dir).join("cv_data.rs");
    let mut file = File::open("cv_data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    std::fs::write(
        &cv_data_path,
        format!("const RAW_CV_DATA: &str = r#\"{data}\"#;"),
    )
    .unwrap();

    // Generate avatar.rs with binary data
    let avatar_path = std::path::Path::new(&out_dir).join("avatar.rs");
    let mut avatar_file = File::open("avatar.mp4").unwrap();
    let mut avatar_data = Vec::new();
    avatar_file.read_to_end(&mut avatar_data).unwrap();

    let avatar_bytes = avatar_data
        .iter()
        .map(|b| format!("{}", b))
        .collect::<Vec<String>>()
        .join(", ");

    std::fs::write(
        &avatar_path,
        format!("pub const AVATAR_MP4_DATA: &[u8] = &[{}];", avatar_bytes),
    )
    .unwrap();
}
