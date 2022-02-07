use std::collections::HashMap;

fn validate(rec: &&HashMap<String, String>) -> bool {
    lib::REQUIRED
        .iter()
        .map(|(key, _)| key)
        .all(|&key| rec.contains_key(key))
}

fn main() {
    let data = lib::read_data();
    println!("{}", data.iter().filter(validate).count());
}
