fn main() {
    // 1. Define the config structure.
    let config_const_values = {
        // 2. Get the path of the config file from the command line.
        let config_path = std::env::args().nth(1).unwrap();

        // 3. Load the whole file contents into a string.
        let config_text = std::fs::read_to_string(&config_path).unwrap();

        // 4. Load an unmutable config structure from the string.
        config_text.parse::<toml::Value>().unwrap()
    };

    // 5. Show the whole config structure.
    println!("Original: {:#?}", config_const_values);

    // 6. Get and show one config value.
    println!(
        "[Postgresql].Database: {}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}
