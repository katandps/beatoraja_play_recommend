use schema::account;

fn main() {
    let schema = schemars::schema_for!(account::AccountResponse);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
