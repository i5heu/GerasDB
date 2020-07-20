use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, NO_PARAMS};
use std::sync::{Arc, Mutex};

pub fn init(pool: &Pool<SqliteConnectionManager>) -> Result<(), rusqlite::Error> {
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS persistentStore(
            hash CHARACTER(97) primary key,
            key CHARACTER(97),
            tree_hash CHARACTER(97) NOT NULL UNIQUE,
            parent_hash CHARACTER(97) NOT NULL UNIQUE,
            lvl UNSIGNED INTEGER NOT NULL, 
            creator CHARACTER(97) NOT NULL,
            created INTEGER NOT NULL,
            importance UNSIGNED INTEGER NOT NULL,
            content TEXT, 
            deleted BOOLEAN, 
            hash_if_deleted CHARACTER(97) NOT NULL UNIQUE, 
            last_checked INTEGER,
            reading_errors UNSIGNED INTEGER,
            extras TEXT
          )",
            NO_PARAMS,
        )
        .unwrap();

    Ok(())
}
