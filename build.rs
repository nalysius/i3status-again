fn main() {
    println!("cargo:rustc-check-cfg=cfg(sndio)");

    let target = std::env::var("TARGET").unwrap_or_default();
    let feature_sndio = std::env::var_os("CARGO_FEATURE_SNDIO").is_some();

    // Link (-l) against sndio on OpenBSD or if the --features sndio flag
    // is given.
    if target.contains("openbsd") || feature_sndio {
        println!("cargo:rustc-link-lib=sndio");
        println!("cargo:rustc-cfg=sndio");
    }
}
