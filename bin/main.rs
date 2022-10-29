use clap::{App, Arg, SubCommand};

mod install;
mod logger;
mod new;
mod print;

use install::*;
use new::*;
use print::*;

fn main() {
    let matches = App::new("Comet")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Isomorphic web framework for Rust")
        .subcommand(SubCommand::with_name("build").about("Build the current project directory"))
        .subcommand(SubCommand::with_name("run").about("Run the current project directory"))
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new empty project folder")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("The name of the new project"),
                ),
        )
        .get_matches();

    logger::init_logger();

    if let Some(_matches) = matches.subcommand_matches("build") {
        build();
    } else if let Some(_matches) = matches.subcommand_matches("run") {
        run();
    } else if let Some(matches) = matches.subcommand_matches("new") {
        create_project_folder(matches.value_of("name").unwrap());
    } else {
        println!("{}", matches.usage());
    }
}

fn build() {
    check_and_install_wasm_pack();
    check_and_install_diesel_cli();

    log_execute(
        "Building client",
        "wasm-pack",
        &[
            "--log-level",
            "warn",
            "build",
            "--target",
            "web",
            "--out-dir",
            "dist/pkg",
            "--",
            "--color",
            "always",
            "-q",
        ],
    );

    log_execute(
        "Building server",
        "cargo",
        &["--color", "always", "build", "-q"],
    );
}

fn run() {
    build();

    log_execute_async("Running", "cargo", &["--color", "always", "run", "-q"]);
}
