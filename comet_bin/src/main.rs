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
    println!("Build");
    println!("[ ] Building server");

    println!(
        "{}",
        String::from_utf8(
            Command::new("cargo")
                .args(["build"])
                .output()
                .expect("failed to run cargo build")
                .stderr
        )
        .unwrap()
    );

    println!("[ ] Building client");

    println!(
        "{}",
        String::from_utf8(
            Command::new("wasm-pack")
                .args(["build", "--target", "web"])
                .output()
                .expect("failed to run npm run build")
                .stderr
        )
        .unwrap()
    );
}

fn run() {
    build();

    println!("Run");
    println!("[ ] Running server");

    println!(
        "{}",
        String::from_utf8(
            Command::new("cargo")
                .args(["run"])
                .output()
                .expect("failed to run cargo run")
                .stderr
        )
        .unwrap()
    );

    println!("[ ] Running client");

    Command::new("http")
        .args(["-p", "8080"])
        .output()
        .expect("failed to run http");
}

fn create_project_folder(_name: &str) {}
