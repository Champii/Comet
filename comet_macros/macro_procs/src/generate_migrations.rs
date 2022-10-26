use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::{Arc, RwLock};

use syn::{parse::Result, Fields, ItemStruct};

lazy_static! {
    pub static ref MIGRATIONS: Arc<RwLock<Vec<(String, String)>>> =
        Arc::new(RwLock::new(Vec::new()));
}

pub fn perform(input: TokenStream) -> TokenStream {
    // let item = parse_macro_input!(input as syn::ItemStruct);

    generate_migrations();

    input
}

fn generate_migrations() {
    let migrations = MIGRATIONS.read().unwrap();

    let mut up_res = String::new();
    let mut down_res = String::new();

    for (up, down) in migrations.iter() {
        up_res += up;
        down_res += down;
    }

    std::fs::create_dir_all("migrations/dev/").unwrap();

    let old_dev_up_migration =
        std::fs::read_to_string("migrations/dev/up.sql").unwrap_or("".to_string());

    if old_dev_up_migration != up_res {
        println!(" -> Detected a change in the database schema, generating a new migration");

        std::fs::write("migrations/dev/up.sql", up_res.to_string()).unwrap();
        std::fs::write("migrations/dev/down.sql", down_res.to_string()).unwrap();

        std::fs::remove_file("src/schema.rs").unwrap();

        std::process::Command::new("diesel")
            .arg("setup")
            .output()
            .expect("failed to execute process");

        // execute diesel migrations redo
        std::process::Command::new("diesel")
            .arg("database")
            .arg("reset")
            .output()
            .expect("failed to execute process");

        std::process::Command::new("diesel")
            .arg("migration")
            .arg("run")
            .output()
            .expect("failed to execute process");

        // replace 'diesel' with 'crate::diesel' in the file src/schema.rs with sed
        std::process::Command::new("sed")
            .arg("-i")
            .arg("s/^diesel::table/crate::diesel::table/g")
            .arg("src/schema.rs")
            .output()
            .expect("failed to execute process");
    }

    /* // execute diesel migrations redo
    let output = std::process::Command::new("diesel")
        .arg("migration")
        .arg("redo")
        .output()
        .expect("failed to execute process");

    // replace 'diesel' with 'crate::diesel' in the file src/schema.rs with sed
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg("s/^diesel::table/crate::diesel::table/g")
        .arg("src/schema.rs")
        .output()
        .expect("failed to execute process"); */
}

pub fn struct_to_sql(obj: ItemStruct) -> Result<(String, String)> {
    let data = obj.fields;

    let fields = match data {
        Fields::Named(fields) => fields.named,
        _ => panic!("Only named fields are supported"),
    };

    let sql_fields = fields
        .iter()
        .map(|field| {
            format!(
                "{} {} NOT NULL",
                field.ident.clone().unwrap().to_string(),
                rust_to_diesel_type(field.ty.clone()),
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let up = format!(
        "CREATE TABLE {}s (
                id SERIAL PRIMARY KEY,
                {sql_fields}
            );",
        obj.ident.to_string().to_ascii_lowercase(),
    );

    let down = format!(
        "DROP TABLE {}s;",
        obj.ident.to_string().to_ascii_lowercase()
    );

    Ok((up, down))
}

fn rust_to_diesel_type(rust_ty: syn::Type) -> String {
    match rust_ty {
        syn::Type::Path(path) => {
            let path = path.path;
            let path = path.segments;
            let path = path
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<String>>();
            let path = path.join("::");

            match path.as_str() {
                "i32" => "INTEGER".to_string(),
                "i64" => "BIGINT".to_string(),
                "String" => "TEXT".to_string(),
                "bool" => "BOOLEAN".to_string(),
                "f32" => "REAL".to_string(),
                "f64" => "DOUBLE PRECISION".to_string(),
                "chrono::NaiveDateTime" => "TIMESTAMP".to_string(),
                "uuid::Uuid" => "UUID".to_string(),
                _ => panic!("Unsupported type: {}", path),
            }
        }
        _ => panic!("Unsupported type"),
    }
}

pub fn register_migration(obj: ItemStruct) {
    let sql = struct_to_sql(obj).unwrap();
    MIGRATIONS.write().unwrap().push(sql);
}
