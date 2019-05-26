use serde_json::{Number, Value};

fn main() {
    // Get the filenames from the command line.
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();

    let mut sales_and_products = {
        // Load the first file into a string.
        let sales_and_products_text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&sales_and_products_text).unwrap()
    };

    // Get the field of the structure
    // containing the weight of the sold oranges.
    if let Value::Number(n) = &sales_and_products["sales"][1]["quantity"] {
        // Increment it and store it back into the structure.
        sales_and_products["sales"][1]["quantity"] =
            Value::Number(Number::from_f64(n.as_f64().unwrap() + 1.5).unwrap());
    }

    // Save the JSON structure into the other file.
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&sales_and_products).unwrap(),
    )
    .unwrap();
}
