use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub(crate) fn create_project_folder(name: &str) {
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
    create_file(
        "src/main.rs",
        &r#"use comet::prelude::*;

#[tokio::main]
pub async fn main() {
    {{name}}::main().await;
}
        "#
        .replace("{{name}}", name),
    );

    create_file("README.md", "");
}
