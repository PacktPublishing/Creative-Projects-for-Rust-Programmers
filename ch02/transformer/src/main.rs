use serde_derive::{Deserialize, Serialize};
use xml::reader::{EventReader, XmlEvent};
use rusqlite::NO_PARAMS;
extern crate postgres;
use postgres::TlsMode;
use redis::Commands;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Redis {
    host: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Sqlite {
    db_file: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Product {
    id: i32,
    category: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Sale {
    id: String,
    product_id: i32,
    date: i64,
    quantity: f64,
    unit: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

enum LocationItem {
    Other,
    InProduct,
    InSale,
}
enum LocationProduct {
    Other,
    InId,
    InCategory,
    InName,
}
enum LocationSale {
    Other,
    InId,
    InProductId,
    InDate,
    InQuantity,
    InUnit,
}

fn read_json_file(pathname: &str) -> SalesAndProducts {
    serde_json::from_str::<SalesAndProducts>(
        &std::fs::read_to_string(&pathname).unwrap()).unwrap()
}

fn read_xml_file(
    pathname: &str,
    sales_and_products: &mut SalesAndProducts)
{
    let file = std::fs::File::open(pathname).unwrap();
    let file = std::io::BufReader::new(file);
    let mut product: Product = Default::default();
    let mut sale: Sale = Default::default();
    let parser = EventReader::new(file);
    let mut location_item = LocationItem::Other;
    let mut location_product = LocationProduct::Other;
    let mut location_sale = LocationSale::Other;
    for event in parser {
        match &location_item {
            LocationItem::Other => match event {
                Ok(XmlEvent::StartElement { ref name, .. })
                    if name.local_name == "product"
                => {
                    location_item = LocationItem::InProduct;
                    location_product = LocationProduct::Other;
                    product = Default::default();
                },
                Ok(XmlEvent::StartElement { ref name, .. })
                    if name.local_name == "sale"
                => {
                    location_item = LocationItem::InSale;
                    location_sale = LocationSale::Other;
                    sale = Default::default();
                },
                _ => {},
            },
            LocationItem::InProduct => match &location_product {
                LocationProduct::Other =>
                    match event {
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "id" => {
                            location_product = LocationProduct::InId;
                        },
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "category" => {
                            location_product = LocationProduct::InCategory;
                        },
                        Ok(XmlEvent::StartElement { ref name, .. })
                            if name.local_name == "name" => {
                            location_product = LocationProduct::InName;
                        },
                        Ok(XmlEvent::EndElement { .. }) => {
                            location_item = LocationItem::Other;
                            sales_and_products.products.push(product);
                            product = Default::default();
                        },
                        _ => {},
                    },
                LocationProduct::InId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.id = characters.parse::<i32>().unwrap();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    },
                    _ => {},
                },
                LocationProduct::InCategory => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.category = characters.clone();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    },
                    _ => {},
                },
                LocationProduct::InName => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        product.name = characters.clone();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_product = LocationProduct::Other;
                    },
                    _ => {},
                },
            },
            LocationItem::InSale => match &location_sale {
                LocationSale::Other => match event {
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "id" => {
                        location_sale = LocationSale::InId;
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "product-id" => {
                        location_sale = LocationSale::InProductId;
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "date" => {
                        location_sale = LocationSale::InDate;
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "quantity" => {
                        location_sale = LocationSale::InQuantity;
                    },
                    Ok(XmlEvent::StartElement { ref name, .. })
                        if name.local_name == "unit" => {
                        location_sale = LocationSale::InUnit;
                    },
                    Ok(XmlEvent::EndElement { ref name, .. })
                        if name.local_name == "sale" => {
                        location_item = LocationItem::Other;
                        sales_and_products.sales.push(sale);
                        sale = Default::default();
                    },
                    _ => {},
                },
                LocationSale::InId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.id = characters.clone();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    },
                    _ => {},
                },
                LocationSale::InProductId => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.product_id = characters.parse::<i32>().unwrap();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    },
                    _ => {},
                },
                LocationSale::InDate => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.date = characters.parse::<i64>().unwrap();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    },
                    _ => {},
                },
                LocationSale::InQuantity => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.quantity = characters.parse::<f64>().unwrap();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    },
                    _ => {},
                },
                LocationSale::InUnit => match event {
                    Ok(XmlEvent::Characters ( characters ))
                    => {
                        sale.unit = characters.clone();
                    },
                    Ok(XmlEvent::EndElement { .. }) => {
                        location_sale = LocationSale::Other;
                    },
                    _ => {},
                },
            },
        }
    }
}

fn recreate_sqlite_db(sqlite_config: &Sqlite)
-> rusqlite::Result<rusqlite::Connection> {
    let conn = rusqlite::Connection::open(&sqlite_config.db_file)?;
    let _ = conn.execute("DROP TABLE Sales", NO_PARAMS);
    let _ = conn.execute("DROP TABLE Products", NO_PARAMS);
    conn.execute(
        "CREATE TABLE Products (
            id INTEGER PRIMARY KEY,
            category TEXT NOT NULL,
            name TEXT NOT NULL UNIQUE)",
        NO_PARAMS
    )?;
    conn.execute(
        "CREATE TABLE Sales (
            id TEXT PRIMARY KEY,
            product_id INTEGER NOT NULL REFERENCES Products,
            sale_date BIGINT NOT NULL,
            quantity DOUBLE PRECISION NOT NULL,
            unit TEXT NOT NULL)",
        NO_PARAMS
    )?;
    Ok(conn)
}

fn write_into_sqlite_db(
    conn: &rusqlite::Connection,
    sales_and_products: &SalesAndProducts)
    -> rusqlite::Result<()>
{
    for product in &sales_and_products.products {
        conn.execute(
            "INSERT INTO Products (
            id, category, name
            ) VALUES ($1, $2, $3)",
            &[
                &product.id as &rusqlite::types::ToSql,
                &product.category,
                &product.name],
        )?;
    }
    for sale in &sales_and_products.sales {
        conn.execute(
            "INSERT INTO Sales (
            id, product_id, sale_date, quantity, unit
            ) VALUES ($1, $2, $3, $4, $5)",
            &[
                &sale.id as &rusqlite::types::ToSql,
                &sale.product_id,
                &sale.date,
                &sale.quantity,
                &sale.unit],
        )?;
    }
    Ok(())
}

fn recreate_postgresql_db(postgresql_config: &Postgresql)
    -> postgres::Result<postgres::Connection>
 {
    let conn = postgres::Connection::connect(
        format!(
            "postgres://{}{}{}@{}{}{}{}{}",
            postgresql_config.username,
            if postgresql_config.password.len() == 0 { "" } else { ":" },
            postgresql_config.password,
            postgresql_config.host,
            if postgresql_config.port.len() == 0 { "" } else { ":" },
            postgresql_config.port,
            if postgresql_config.database.len() == 0 { "" } else { "/" },
            postgresql_config.database),
        TlsMode::None)?;
    conn.execute("DROP TABLE Sales", &[])?;
    conn.execute("DROP TABLE Products", &[])?;
    conn.execute(
        "CREATE TABLE Products (
        id INTEGER PRIMARY KEY,
        category TEXT NOT NULL,
        name TEXT NOT NULL UNIQUE)",
        &[]
    )?;
    conn.execute(
        "CREATE TABLE Sales (
        id TEXT PRIMARY KEY,
        product_id INTEGER NOT NULL REFERENCES Products,
        sale_date BIGINT NOT NULL,
        quantity DOUBLE PRECISION NOT NULL,
        unit TEXT NOT NULL)",
        &[]
    )?;
    Ok(conn)
}

fn write_into_postgresql_db(
    conn: &postgres::Connection,
    sales_and_products: &SalesAndProducts)
    -> postgres::Result<()>
{
    for product in &sales_and_products.products {
        conn.execute(
            "INSERT INTO Products (
            id, category, name
            ) VALUES ($1, $2, $3)",
            &[
                &(product.id as i32) as &postgres::types::ToSql,
                &product.category,
                &product.name],
        )?;
    }
    for sale in &sales_and_products.sales {
        conn.execute(
            "INSERT INTO Sales (
            id, product_id, sale_date, quantity, unit
            ) VALUES ($1, $2, $3, $4, $5)",
            &[
                &sale.id as &postgres::types::ToSql,
                &(sale.product_id as i32),
                &sale.date,
                &sale.quantity,
                &sale.unit],
        )?;
    }
    Ok(())
}

fn write_into_redis_db(
    redis_config: &Redis,
    sales_and_products: &SalesAndProducts)
    -> redis::RedisResult<()>
{
    let conn = redis::Client::open(
        format!("redis://{}/", redis_config.host).as_str())?
        .get_connection()?;

    for product in &sales_and_products.products {
        conn.set(format!("product:{}:category", product.id),
            &product.category)?;
        conn.set(format!("product:{}:name", product.id),
            &product.name)?;
    }
    for sale in &sales_and_products.sales {
        conn.set(format!("sale:{}:product_id", sale.id),
            sale.product_id)?;
        conn.set(format!("sale:{}:sale_date", sale.id),
            sale.date)?;
        conn.set(format!("sale:{}:quantity", sale.id),
            sale.quantity)?;
        conn.set(format!("sale:{}:unit", sale.id),
            &sale.unit)?;
    }
    Ok(())
}

fn main() {
    // Define the config structure by reading the TOML file
    // specified in the command line.
    let config: Config =
    {
        let config_path = std::env::args().nth(1).unwrap();
        let config_text = std::fs::read_to_string(&config_path).unwrap();
        toml::from_str(&config_text).unwrap()
    };

    let mut sales_and_products = 
        read_json_file(&config.input.json_file);

    read_xml_file(&config.input.xml_file, &mut sales_and_products);

    let sqlite_conn = recreate_sqlite_db(&config.sqlite).unwrap();
    write_into_sqlite_db(&sqlite_conn, &sales_and_products).unwrap();

    let postgresql_conn = recreate_postgresql_db(&config.postgresql).unwrap();
    write_into_postgresql_db(&postgresql_conn, &sales_and_products).unwrap();

    write_into_redis_db(&config.redis, &sales_and_products).unwrap();
}
