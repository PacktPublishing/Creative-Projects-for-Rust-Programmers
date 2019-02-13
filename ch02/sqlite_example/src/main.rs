use rusqlite::{Connection, Result, NO_PARAMS};
use rusqlite::types::ToSql;

#[derive(Debug)]
struct SaleWithProduct {
    category: String,
    name: String,
    quantity: f64,
    unit: String,
    date: i64,
}

fn create_db() -> Result<Connection> {
    let database_file = "sales.db";
    let conn = Connection::open(database_file)?;
    /*
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Products (
             id INTEGER PRIMARY KEY,
             category TEXT NOT NULL,
             name TEXT NOT NULL UNIQUE
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Sales (
             id TEXT PRIMARY KEY,
             product_id INTEGER NOT NULL,
             sale_date INTEGER NOT NULL,
             quantity REAL NOT NULL,
             unit TEXT NOT NULL,
             CONSTRAINT FK_Product
                FOREIGN KEY (product_id)
                REFERENCES Products (id)
         )",
        NO_PARAMS,
    )?;
    */
    conn.execute("DROP TABLE Sales", NO_PARAMS)?;
    conn.execute("DROP TABLE Products", NO_PARAMS)?;
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

fn populate_db(conn: &Connection) -> Result<()> {
    /*
    conn.execute(
        "INSERT INTO Products (
            id, category, name
            ) VALUES (?1, ?2, ?3)",
        &[&1 as &ToSql, &"Frutta", &"Mele"],
    )?;
    conn.execute(
        "INSERT INTO Sales (
            id, product_id, date, quantity, unit
            ) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&"2019-001" as &ToSql, &1,
            &1234567890, &7.439, &"Kg"],
    )?;
    Ok(())
    */
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
    let mut command = conn.prepare(
        "SELECT p.name, s.unit, s.quantity, s.sale_date
        FROM Sales s
        LEFT JOIN Products p
        ON p.id = s.product_id
        ORDER BY s.sale_date")?;
    for sale_with_product in command.query_map(NO_PARAMS, |row|
        SaleWithProduct {
            category: "".to_string(),
            name: row.get(0),
            quantity: row.get(2),
            unit: row.get(1),
            date: row.get(3)})?
    {
        if let Ok(item) = sale_with_product { 
            println!("At instant {}, {} {} of {} were sold.",
                item.date,
                item.quantity,
                item.unit,
                item.name);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let conn = create_db()?;
    populate_db(&conn)?;
    print_db(&conn)?;
    Ok(())
}
