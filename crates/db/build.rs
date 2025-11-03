fn main() {
    // Tell Cargo to rerun this build script if queries directory changes
    println!("cargo:rerun-if-changed=queries");
}
