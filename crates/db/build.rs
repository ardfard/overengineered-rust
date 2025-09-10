use std::{env, path::Path};

fn main() {
    cornucopia();
}


fn cornucopia(){
    let queries_path = "queries";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    println!("cargo:rerun-if-changed={queries_path}");

    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("Failed to run cornucopia: {}", String::from_utf8_lossy(&output.stderr));
    }
}