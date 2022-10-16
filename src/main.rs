use std::process::Command;

use clap::{App, Arg, SubCommand};

mod logger;

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
    Command::new("cargo")
        .args(["build"])
        .output()
        .expect("failed to run cargo build");

    Command::new("wasm-pack")
        .args(["build", "--target", "web"])
        .output()
        .expect("failed to run wasm-pack build");
}

fn run() {
    Command::new("cargo")
        .args(["run"])
        .output()
        .expect("failed to run cargo run");

    Command::new("http")
        .args(["-p", "8080"])
        .output()
        .expect("failed to run http");
}

fn create_project_folder(_name: &str) {}
