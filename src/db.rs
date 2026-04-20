use rusqlite::Connection;

pub fn init_db() -> Result<(), rusqlite::Error> {
    let connection = Connection::open("tickets.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            priority TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
