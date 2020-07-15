use db_layer::PersistentItem;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
pub use rusqlite::Error as dbError;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::process;
mod db_layer;
mod initialize_db;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub struct DbSession {
    pub db_name: String,
    pub pool: Pool<SqliteConnectionManager>,
}

pub fn init() -> Result<DbSession, rusqlite::Error> {
    let manager = SqliteConnectionManager::file(":memory:")
        .with_init(|c| c.execute_batch("PRAGMA foreign_keys=1;"));
    let pool = r2d2::Pool::new(manager).unwrap();

    match initialize_db::init(&pool) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let bar = DbSession {
        db_name: "HelloDBName".to_owned(),
        pool,
    };
    // return bar;
    Ok(bar)
}

#[test]
fn set_and_get_test() -> Result<(), rusqlite::Error> {
    let result = init()?;

    let hash = "83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b";

    let test_item: &PersistentItem = &PersistentItem {
        hash: String::from(hash),
        tree_hash: String::from(hash),
        parent_hash: String::from(hash),
        hash_if_deleted: String::from(hash),
        lvl: 456835687,
        creator: String::from(hash),
        created: 567445672,
        importance: 234235675,
        content: String::from(hash),
        deleted: false,
        last_checked: 2141235,
        reading_errors: 235235,
        extras: String::from(hash),
    };

    let _ = db_layer::insert(&result.pool, &test_item)?;

    let get_result = db_layer::get(&result.pool, &test_item.hash)?;

    assert_eq!(hash, get_result.hash);
    assert_eq!(hash, get_result.content);
    assert_eq!(2141235, get_result.last_checked);
    assert_eq!(hash, get_result.extras);

    Ok(())
}
