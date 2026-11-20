//! Windows only: embed the app icon and version info (VERSIONINFO) into `terralauncher.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `TERRALAUNCHER_REQUIRE_WINRES=1` (set
//! by the release packaging) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/terralauncher.ico");
    println!("cargo:rerun-if-env-changed=TERRALAUNCHER_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/terralauncher.ico")
        .set("ProductName", "terraLauncher")
        .set("FileDescription", "terraLauncher")
        .set("CompanyName", "terraPen")
        .set("LegalCopyright", "Copyright (c) the terraLauncher authors and the terraLauncher contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "terralauncher.exe")
        .set("InternalName", "terralauncher");
    if let Err(e) = res.compile() {
        if std::env::var_os("TERRALAUNCHER_REQUIRE_WINRES").is_some() {
            println!("cargo:warning=embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=terralauncher.exe built without icon/version resources: {e}");
    }
}
