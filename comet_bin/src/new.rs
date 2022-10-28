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
        "src/main.rs",
        r#"// The mandatory imports
use comet::prelude::*;

// This macro takes two arguments:
// - A type for which we will implement `Component`
// - And a root HTML element
// We implement `Component` for a simple integer.
component! {
    // We use an i32 here, but you can use any stucts/enums/custom type
    i32,

    // The root of this HTML element is a simple button
    // It has a 'click' event registered that will increment our i32 by 1
    button @click: { *self += 1 } {
        // We display our value inside the button
        { self }
    }
}

// This is where all the magic happens
// We run the application with an instance of our i32 component that starts with the value 0
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
path = "src/main.rs"
crate-type = ["cdylib", "rlib"]

[[bin]]
path = "src/main.rs"
name = "comet_test"

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

    create_file("README.md", "");
}
