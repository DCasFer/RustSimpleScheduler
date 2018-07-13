fn main() {

    println!("cargo:rustc-link-search=./extraLibs");

    println!("cargo:rustc-link-lib=static=scheduler");

    println!("cargo:rustc-link-lib=static=memset");

    println!("cargo:rustc-link-lib=static=contextSwitch");
}
