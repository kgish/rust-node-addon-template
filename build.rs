// Placing a file named build.rs in the root of a package will cause Cargo to compile that script
// and execute it just before building the package.
fn main() {
    println!("cargo:rustc-cdylib-link-arg=-undefined");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}
