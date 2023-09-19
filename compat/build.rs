fn main() {
    println!("cargo:rerun-if-changed=src/CMakeLists.txt");

    let dest = cmake::build("src");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=libpci-rs-c-backend");

    println!("cargo:rerun-if-changed=src/api.h");

    let bindings = bindgen::Builder::default()
        .header("src/api.h")
        .rustified_enum("pci_error")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");

    bindings
        .write_to_file("bindings.rs")
        .expect("Unable to write bindings.");
}