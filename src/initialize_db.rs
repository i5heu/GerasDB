use rusqlite::{Connection, NO_PARAMS};
use std::sync::Arc;

pub fn init(conn: &Arc<Connection>) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE persistentStore(
            hash CHARACTER(97) primary key,
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
    )?;

    Ok(())
}

