pub use rusqlite::Error as dbError;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

pub struct Foo {
    db_name: String,
    conn: rusqlite::Connection,
}

pub fn init() -> Result<Foo, rusqlite::Error> {
    let conn = Connection::open(":memory:")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    let bar = Foo {
        db_name: "Hello".to_owned(),
        conn,
    };
    // return bar;
    Ok(bar)
}

#[test]
fn initialization_test() -> Result<(), rusqlite::Error> {
    init()?;
    Ok(())
}
