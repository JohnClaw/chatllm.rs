fn main() {
    println!("cargo:rustc-link-search=native=C:\\rst");
    println!("cargo:rustc-link-lib=dylib=libchatllm");
}