extern crate postgres;

use postgres::{Connection, TlsMode, Result};
use postgres::types::ToSql;

#[derive(Debug)]
struct SaleWithProduct {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}

fn create_db() -> Result<Connection> {
    let username = "postgres";
    let password = "post";
    let host = "localhost";
    let port = "5432";
    let database = "Rust2018";
    let conn = Connection::connect(
        format!(
            "postgres://{}{}{}@{}{}{}{}{}",
            username,
            if password.len() == 0 { "" } else { ":" },
            password,
            host,
            if port.len() == 0 { "" } else { ":" },
            port,
            if database.len() == 0 { "" } else { "/" },
            database),
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

fn populate_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO Products (
            id, category, name
            ) VALUES ($1, $2, $3)",
        &[&1 as &ToSql, &"fruit", &"pears"],
    )?;
    conn.execute(
        "INSERT INTO Sales (
            id, product_id, sale_date, quantity, unit
            ) VALUES ($1, $2, $3, $4, $5)",
        &[&"2020-183" as &ToSql, &1,
            &1234567890i64, &7.439, &"Kg"],
    )?;
    Ok(())
}

fn print_db(conn: &Connection) -> Result<()> {
    for row in &conn.query(
        "SELECT p.name, s.unit, s.quantity, s.sale_date
        FROM Sales s
        LEFT JOIN Products p
        ON p.id = s.product_id
        ORDER BY s.sale_date",
        &[])?
    {
        let sale_with_product = SaleWithProduct {
            category: "".to_string(),
            name: row.get(0),
            quantity: row.get(2),
            unit: row.get(1),
            date: row.get(3),
        };
        println!("At instant {}, {} {} of {} were sold.",
            sale_with_product.date,
            sale_with_product.quantity,
            sale_with_product.unit,
            sale_with_product.name);
    }
    Ok(())
}

fn main() -> Result<()> {
    let conn = create_db()?;
    populate_db(&conn)?;
    print_db(&conn)?;
    Ok(())
}
