fn main() {
    // Tauri build hook
    tauri_build::build();

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
