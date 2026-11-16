use std::{env, fs, path::PathBuf};

fn main() {
    // This is the same import-library path used by the OGRE bridge CMake project.
    let bridge_dir = PathBuf::from(r"C:\\OgreSDK\\ogre_bridge");
    let release_dir = bridge_dir.join("build").join("Release");
    let bridge_cpp = bridge_dir.join("ogre_bridge.cpp");
    let bridge_lib = release_dir.join("ogre_bridge.lib");
    let bridge_dll = release_dir.join("ogre_bridge.dll");

    println!("cargo:rerun-if-changed={}", bridge_cpp.display());
    println!("cargo:rerun-if-changed={}", bridge_dll.display());
    println!("cargo:rustc-link-search=native={}", release_dir.display());
    println!("cargo:rustc-link-lib=dylib=ogre_bridge");

    if !bridge_cpp.is_file() {
        panic!("OGRE bridge source not found: {}. Extract ogre_bridge.zip into C:\\OgreSDK first.", bridge_cpp.display());
    }
    if !bridge_lib.is_file() || !bridge_dll.is_file() {
        panic!("OGRE bridge is not built. Run C:\\OgreSDK\\ogre_bridge\\BUILD_BRIDGE.cmd before cargo build.");
    }

    let source_time = fs::metadata(&bridge_cpp)
        .and_then(|m| m.modified())
        .expect("Cannot read ogre_bridge.cpp timestamp");
    let dll_time = fs::metadata(&bridge_dll)
        .and_then(|m| m.modified())
        .expect("Cannot read ogre_bridge.dll timestamp");
    if source_time > dll_time {
        panic!("ogre_bridge.dll is older than ogre_bridge.cpp. Run BUILD_BRIDGE.cmd, then rebuild the Rust client.");
    }

    // Windows loads a linked DLL from the executable directory. rustc-link-search
    // only finds the import library; it does not deploy the DLL at runtime.
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is missing"));
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Unexpected Cargo OUT_DIR layout (expected target/<profile>/build/<crate>/out)");
    fs::create_dir_all(profile_dir).expect("Cannot create Cargo profile output directory");
    let runtime_dll = profile_dir.join("ogre_bridge.dll");
    fs::copy(&bridge_dll, &runtime_dll).unwrap_or_else(|e| {
        panic!("Failed to copy {} to {}: {}", bridge_dll.display(), runtime_dll.display(), e)
    });
    println!("cargo:warning=OGRE bridge copied beside executable: {}", runtime_dll.display());
}
