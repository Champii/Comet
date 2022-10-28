use std::path::Path;

use which::which;

use super::log_execute;

pub fn check_and_install_diesel_cli() {
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
                "-q",
                "--color",
                "always",
            ],
        );
    }

    if !Path::new("diesel.toml").exists()
        || !Path::new("migrations").exists()
        || !Path::new("src/schema.rs").exists()
    {
        log_execute("Diesel setup", "diesel", &["setup"]);
        log_execute("Reset database", "diesel", &["database", "reset"]);
        log_execute("Migrating database", "diesel", &["migration", "run"]);
        log_execute(
            "Patching schema",
            "sed",
            &["-i", "s/^diesel::/crate::diesel::/g", "src/schema.rs"],
        );
    }
}

pub fn check_and_install_wasm_pack() {
    if which("wasm-pack").is_err() {
        log_execute(
            "Installing wasm-pack",
            "cargo",
            &["install", "wasm-pack", "-q", "--color", "always"],
        );
    }
}
