fn main() {
    let config_app = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());
    let config_dialog = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());

    println!("cargo:warning================================== compile build.rs =================================\n");
    if slint_build::compile_with_config("ui/app.slint", config_app).is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_APP={}/app.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/app.slint success\n");
    }
    if slint_build::compile_with_config("ui/dialog.slint", config_dialog).is_ok() {
        println!(
            "cargo:rustc-env=SLINT_INCLUDE_DIALOG={}/dialog.rs",
            std::env::var("OUT_DIR").unwrap()
        );
        println!("cargo:warning=compile ui/dialog.slint success\n");
    }
}