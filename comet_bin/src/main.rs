use colored::*;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
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

fn print(msg: &str) {
    let mut stdout = std::io::stdout();
    print!("{}", msg);
    stdout.flush().unwrap();
}

fn print_ok(log: &str) {
    println!(
        "\r{}{}{} {}   ",
        "[".purple(),
        "✓".green(),
        "]".purple(),
        log.green()
    );
}

fn print_err(log: &str) {
    println!(
        "\r{}{}{} {}   ",
        "[".purple(),
        "✗".red(),
        "]".purple(),
        log.red()
    );
}

fn print_warn(log: &str) {
    println!(
        "\r{}{}{} {}   ",
        "[".purple(),
        "!".yellow(),
        "]".purple(),
        log.yellow()
    );
}

fn log_execute_async(log: &str, name: &str, args: &[&str]) {
    print(format!("{} {} {}...", "[".purple(), "]".purple(), log).as_str());

    let handle = Command::new(name)
        .env("TERM", "xterm-256color")
        .args(args)
        .stderr(Stdio::null())
        .spawn()
        .expect(&format!("Failed to execute {}", name));

    print_ok(log);

    let status = handle.wait_with_output().unwrap();

    if !status.status.success() {
        std::process::exit(1);
    }
}

fn log_execute(log: &str, name: &str, args: &[&str]) {
    print(format!("{} {} {}...", "[".purple(), "]".purple(), log).as_str());

    let status = Command::new(name)
        .env("TERM", "xterm-256color")
        .args(args)
        .output()
        .expect(&format!("Failed to execute {}", name));

    let out = String::from_utf8_lossy(&status.stderr);

    if out.trim_end().is_empty() {
        print_ok(log);
    } else {
        if status.status.success() {
            print_warn(log);
        } else {
            print_err(log);
        }

        println!("{}", out);
    }

    if !status.status.success() {
        std::process::exit(1);
    }
}

fn build() {
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

fn create_project_folder(name: &str) {
    let path = Path::new(name);

    if path.exists() {
        println!("Error: {} already exists", name);

        return;
    }

    fs::create_dir(path).expect("Failed to create project folder");
    fs::create_dir(path.join("src")).expect("Failed to create project src folder");
    fs::create_dir(path.join("dist")).expect("Failed to create project dist folder");

    let create_file = |new_path: &str, content: &str| {
        let mut file = File::create(path.join(new_path)).expect("Failed to create file");
        file.write(content.as_bytes()).unwrap();
    };

    create_file(
        "src/lib.rs",
        r#"use comet::prelude::*;

component! {
    i32,
    button @click: { *self += 1 } {
        { self }
    }
}

comet!(0);
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
        "dist/index.html",
        &r#"<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type="module">
      import init from './assets/pkg/{{name}}.js';

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
