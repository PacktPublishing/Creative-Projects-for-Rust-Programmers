use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

fn main() -> Result<(), std::io::Error> {
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();
    let mut sales_and_products = {
        let sales_and_products_text = std::fs::read_to_string(&input_path)?;

        // 1. Load the sale structure from the string.
        serde_json::from_str::<SalesAndProducts>(&sales_and_products_text).unwrap()
    };

    // Increment the weight of the sold oranges.
    sales_and_products.sales[1].quantity += 1.5;

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&sales_and_products).unwrap(),
    )?;

    Ok(())
}
