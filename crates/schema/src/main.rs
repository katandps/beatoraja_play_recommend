use schema::account;
use schemars::schema_for;

fn main() {
    let schemas = vec![("account", schema_for!(account::AccountResponse))];

    for (name, schema) in schemas {
        write_schema_file(&schema, "target/schemas", &format!("{}.json", name));
    }
}

fn write_schema_file(schema: &schemars::Schema, out_dir: &str, file_name: &str) {
    use serde_json::to_string_pretty;
    use std::fs;
    use std::path::Path;

    let out_path = format!("{}/{}", out_dir, file_name);
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Failed to create output directory");
    }
    let json = to_string_pretty(schema).expect("Failed to serialize schema");
    fs::write(&out_path, json).expect("Failed to write schema file");
    println!("Schema written to {}", out_path);
}
