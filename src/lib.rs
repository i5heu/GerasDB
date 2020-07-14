pub use rusqlite::Error as dbError;
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::{process, sync::Arc};
mod initialize_db;
pub struct Foo {
    pub db_name: String,
    pub conn: Arc<rusqlite::Connection>,
}

pub fn init() -> Result<Foo, rusqlite::Error> {
    let conn = Arc::new(Connection::open(":memory:")?);

    match initialize_db::init(&conn) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let bar = Foo {
        db_name: "HelloDBName".to_owned(),
        conn,
    };
    // return bar;
    Ok(bar)
}

#[test]
fn initialization_test() -> Result<(), rusqlite::Error> {
    let result = init()?;

    result.conn.execute(
        "INSERT INTO persistentStore (hash,treeHash,parentHash,hashIfDeleted,level,creator,created,importance,deleted,lastChecked,readingErrors,extras) VALUES (
        '83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b',
        '83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b',
        '83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b',
        '83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b',
        '0',
        '83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b',
        '1594755983',
        '24124',
        'false',
        '1594755983',
        'LOREM iPsUM!',
        '{\"test\":\"hello world\"}'
      );",
      NO_PARAMS 
    )?;

    struct TestHash {
        hash: String,
    }

    let mut stmt = result.conn.prepare("SELECT hash FROM persistentStore")?;
    let hash_iter = stmt.query_map(params![], |row| Ok(TestHash { hash: row.get(0)? }))?;

    for hash in hash_iter {
        let boo = match hash {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

        assert_eq!(boo.hash, "83bff28dde1b1bf5810071c6643c08e5b05bdb836effd70b403ea8ea0a634dc4997eb1053aa3593f590f9c63630dd90b")
    }

    Ok(())
}
