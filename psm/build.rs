fn main() {
    println!("cargo:rustc-check-cfg=cfg(switchable_stack,asm,link_asm)");
    if std::env::var_os("CARGO_CFG_MIRI").is_some() {
        return;
    }
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // Symbols are provided by src/asm.rs (global_asm), not by a C toolchain.
    if !matches!(arch.as_str(), "x86_64" | "x86" | "aarch64") {
        return;
    }
    println!("cargo:rustc-cfg=asm");
    // Windows grows stacks with fibers. x86_64 and aarch64 switch stacks here.
    if os != "windows" && matches!(arch.as_str(), "x86_64" | "aarch64") {
        println!("cargo:rustc-cfg=switchable_stack");
    }
}
