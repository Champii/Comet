use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

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

fn create_project_folder(name: &str) {
    let path = Path::new(name);

    if path.exists() {
        println!("Error: {} already exists", name);

        return;
    }

    fs::create_dir(path).expect("Failed to create project folder");
    fs::create_dir(path.join("src")).expect("Failed to create project src folder");

    let create_file = |new_path: &str, content: &str| {
        let mut file = File::create(path.join(new_path)).expect("Failed to create file");
        file.write(content.as_bytes()).unwrap();
    };

    create_file(
        "src/lib.rs",
        r#"use comet::prelude::*;

#[derive(Default)]
pub struct Counter {
    pub value: i32,
}

component! { Counter,
    button @click: { self.value += 1 } {
        {{ self.value }}
    }
}

comet!(Counter::default());
"#,
    );

    create_file(
        "Cargo.toml",
        &r#"[package]
name = "{{name}}"
version = "0.1.0"
edition = "2021"


[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
comet = { git = "https://github.com/Champii/Comet" }
        "#
        .replace("{{name}}", name),
    );

    create_file(
        "index.html",
        &r#"<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type="module">
      import init from './pkg/{{name}}.js';

      async function run() {
        await init();
      }
      run();
    </script>
  </body>
</html>
        "#
        .replace("{{name}}", name),
    );
}
