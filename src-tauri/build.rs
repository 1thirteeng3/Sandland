fn main() {
    if std::env::var("CARGO_FEATURE_APP").is_ok() {
        // Tauri build hook - on Windows GNU (MinGW), use without_app_manifest to prevent ld.exe .rsrc merge failure
        #[cfg(all(windows, target_env = "gnu"))]
        {
            let attrs = tauri_build::Attributes::new()
                .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());
            tauri_build::try_build(attrs).expect("failed to run tauri-build");
        }
        #[cfg(not(all(windows, target_env = "gnu")))]
        {
            tauri_build::build();
        }
    }

    // Statically compile sqlite-vec C amalgamation
    let sqlite_vec_path = std::path::Path::new("vendor/sqlite-vec/sqlite-vec.c");
    if sqlite_vec_path.exists() {
        println!("cargo:rerun-if-changed=vendor/sqlite-vec/sqlite-vec.c");
        println!("cargo:rerun-if-changed=vendor/sqlite-vec/sqlite-vec.h");

        let mut build = cc::Build::new();
        build.file("vendor/sqlite-vec/sqlite-vec.c");
        build.include("vendor/sqlite-vec");
        build.flag_if_supported("-std=c99");
        build.flag_if_supported("-O3");
        build.warnings(false);
        
        // Define SQLITE_VEC_STATIC to prevent export conflict
        build.define("SQLITE_CORE", "1");
        
        build.compile("sqlite_vec");
    }
}
