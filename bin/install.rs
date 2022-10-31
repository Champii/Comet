use std::path::Path;

use which::which;

use super::log_execute;

pub fn check_and_install_diesel_cli(verbose: bool) {
    if which("diesel").is_err() {
        log_execute(
            "Installing diesel-cli",
            "cargo",
            &[
                "install",
                "diesel_cli",
                "--no-default-features",
                "--features",
                "postgres",
                "--color",
                "always",
            ],
            verbose,
        );
    }

    if !Path::new("diesel.toml").exists()
        || !Path::new("migrations").exists()
        || !Path::new("src/schema.rs").exists()
    {
        log_execute("Diesel setup", "diesel", &["setup"], true);
        log_execute("Reset database", "diesel", &["database", "reset"], true);
        log_execute("Migrating database", "diesel", &["migration", "run"], true);
        log_execute(
            "Patching schema",
            "sed",
            &["-i", "s/^diesel::/crate::diesel::/g", "src/schema.rs"],
            true,
        );
    }
}

pub fn check_and_install_wasm_pack(verbose: bool) {
    if which("wasm-pack").is_err() {
        log_execute(
            "Installing wasm-pack",
            "cargo",
            &["install", "wasm-pack", "--color", "always"],
            verbose,
        );
    }
}
