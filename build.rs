fn main() {
    println!("cargo:rerun-if-changed=ffi/tun.c");
    cc::Build::new().file("ffi/tun.c").compile("tun");
}
