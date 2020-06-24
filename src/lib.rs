use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

pub struct Foo {
    db_name: String,
    conn: rusqlite::Connection,
}

pub fn init() -> Result<Foo> {
    let conn = Connection::open("cats.db")?;

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

pub mod math {
    pub const fn add(a: i8, b: i8) -> i8 {
        return a + b;
    }

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
