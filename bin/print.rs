use colored::*;

use std::{io::Write, process::Command};

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

pub fn log_execute_async(log: &str, name: &str, args: &[&str]) {
    print(format!("{} {} {}...", "[".purple(), "]".purple(), log).as_str());

    let handle = Command::new(name)
        .env("TERM", "xterm-256color")
        .args(args)
        .spawn()
        .expect(&format!("Failed to execute {}", name));

    print_ok(log);

    let status = handle.wait_with_output().unwrap();

    if !status.status.success() {
        std::process::exit(1);
    }
}

pub fn log_execute(log: &str, name: &str, args: &[&str]) {
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
