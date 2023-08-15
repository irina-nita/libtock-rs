fn main() {
    // Support for fixed-address compilation.
    libtock_build_scripts::auto_layout();

    // Configure the alignment size for the linker. This prevents the linker
    // from assuming very large pages (i.e. 65536 bytes) and unnecessarily
    // inserting additional padding into the output ELF.
    println!("cargo:rustc-link-arg=-zmax-page-size=4096");
}
