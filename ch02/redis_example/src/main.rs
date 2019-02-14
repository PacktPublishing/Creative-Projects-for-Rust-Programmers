use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let conn = redis::Client::open("redis://localhost/")?
        .get_connection()?;

    conn.set("aKey", "a string".to_string())?;
    conn.set("anotherKey", 4567)?;
    conn.set(45, 12345)?;

    println!("{}, {}, {}.",
        conn.get::<_, String>("aKey")?,
        conn.get::<_, u64>("anotherKey")?,
        conn.get::<_, u16>(45)?);

    Ok(())
}
