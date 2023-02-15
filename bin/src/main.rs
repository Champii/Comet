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
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the current project directory")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Prints more information while running"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the current project directory")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Prints more information while running"),
                ),
        )
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

    if let Some(matches) = matches.subcommand_matches("build") {
        build(matches.is_present("verbose"));
    } else if let Some(matches) = matches.subcommand_matches("run") {
        run(matches.is_present("verbose"));
    } else if let Some(matches) = matches.subcommand_matches("new") {
        create_project_folder(matches.value_of("name").unwrap());
    } else {
        println!("{}", matches.usage());
    }
}

fn build(verbose: bool) {
    check_and_install_wasm_pack(verbose);
    check_and_install_diesel_cli(verbose);

    log_execute(
        "Building client",
        "wasm-pack",
        &[
            "--log-level",
            "error",
            "build",
            "--target",
            "web",
            "--out-dir",
            "dist/pkg",
            "--dev",
            "--",
            "--color",
            "always",
        ],
        verbose,
    );

    log_execute(
        "Building server",
        "cargo",
        &["--color", "always", "build"],
        verbose,
    );
}

fn run(verbose: bool) {
    build(verbose);

    log_execute_async("Running", "cargo", &["--color", "always", "run"], verbose);
}
