use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dll_source = Path::new(&manifest_dir).join("src").join("rs_cv").join("msvc_x86_64").join("bin");
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let dest_path = Path::new(&manifest_dir).join("target").join(profile);
    let _ = fs::create_dir_all(&dest_path);
    if let Ok(entries) = fs::read_dir(&dll_source) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("dll") {
                let dest_file = dest_path.join(path.file_name().unwrap());
                if let Err(e) = fs::copy(&path, &dest_file) {
                    println!("cargo:warning=DLL nusxalashda xato: {:?}", e);
                }
            }
        }
    }
    println!("cargo:rerun-if-changed=src/RsCv/msvc_x86_64/bin");
}