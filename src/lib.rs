use db_layer::PersistentItem;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
pub use rusqlite::Error as dbError;
use rusqlite::Result;
use std::{
    process,
    time::{Instant, SystemTime},
};
pub mod db_layer;
mod initialize_db;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub struct DbSession {
    pub db_name: String,
    pub pool: Pool<SqliteConnectionManager>,
}

pub fn init() -> Result<DbSession, rusqlite::Error> {
    let manager = SqliteConnectionManager::file("demo.sqlite3")
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
fn set_and_get_by_hash_test() -> Result<(), rusqlite::Error> {
    let result = init()?;

    let bar = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let hash = &bar.as_nanos().to_string();

    let test_item: &PersistentItem = &PersistentItem {
        hash: String::from(hash),
        key: String::from("testing:test"),
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

    let get_result = db_layer::get_by_hash(&result.pool, &test_item.hash)?;

    assert_eq!(String::from(hash), get_result.hash);
    assert_eq!(String::from(hash), get_result.content);
    assert_eq!(2141235, get_result.last_checked);
    assert_eq!(String::from(hash), get_result.extras);

    Ok(())
}
#[test]
fn set_and_get_by_half_key_test() -> Result<(), rusqlite::Error> {
    let result = init()?;

    let bar = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let hash = &bar.as_nanos().to_string();

    let mut key = String::from(hash);
    key.push_str(":test:test:test:test");
    let mut search_key = String::from(hash);
    search_key.push_str(":test:%");

    let test_item: &PersistentItem = &PersistentItem {
        hash: String::from(hash),
        key: String::from(&key),
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
    
    db_layer::insert(&result.pool, &test_item)?;
    let get_result = db_layer::get_by_key(&result.pool, &String::from(&search_key))?;

    assert_eq!(key, get_result.key);

    assert_eq!(String::from(hash), get_result.content);
    assert_eq!(2141235, get_result.last_checked);
    assert_eq!(String::from(hash), get_result.extras);

    Ok(())
}
