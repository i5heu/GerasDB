use rusqlite::{Connection, NO_PARAMS};
use std::sync::Arc;

pub fn init(conn: &Arc<Connection>) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE persistentStore(
            hash CHARACTER(97) primary key,
            treeHash CHARACTER(97) NOT NULL UNIQUE,
            parentHash CHARACTER(97) NOT NULL UNIQUE,
            hashIfDeleted CHARACTER(97) NOT NULL UNIQUE,
            level UNSIGNED INTEGER NOT NULL,
            creator CHARACTER(97) NOT NULL,
            created INTEGER NOT NULL,
            importance UNSIGNED INTEGER NOT NULL,
            content TEXT,
            deleted BOOLEAN,
            lastChecked INTEGER,
            readingErrors UNSIGNED INTEGER,
            extras TEXT
          )",
        NO_PARAMS,
    )?;

    Ok(())
}

