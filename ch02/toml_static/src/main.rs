use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Redis {
    host: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Sqlite {
    db_file: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

fn main() {
    // 1. Define the config structure.
    let config_const_values: Config = {
        // 2. Get the path of the config file from the command line.
        let config_path = std::env::args().nth(1).unwrap();

        // 3. Load the whole file contents into a string.
        let config_text = std::fs::read_to_string(&config_path).unwrap();

        // 4. Load an unmutable statically-typed structure from the string.
        toml::from_str(&config_text).unwrap()
    };

    // 5. Get and show one config value.
    println!(
        "[postgresql].database: {}",
        config_const_values.postgresql.database
    );
}
