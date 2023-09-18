fn main() {
    let dest = cmake::build("src/backend/c");

    println!("cargo:rustc-link-search=native={}", dest.display());
    println!("cargo:rustc-link-lib=static=libpci-rs-c-backend");
}