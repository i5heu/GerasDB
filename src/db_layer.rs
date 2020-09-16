extern crate rustc_serialize;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use std::process;

#[derive(RustcDecodable, RustcEncodable)]
pub struct PersistentItem {
    pub hash: String,
    pub key: String,
    pub tree_hash: String, // random hash defined on the root of the tree,
    pub parent_hash: String,
    pub lvl: u32,        //the level above the tree root,
    pub creator: String, // the creator of this link for diff between sys and usr,
    pub created: i64,    // with ms,
    pub importance: u32, // the importance of the data - higher is more important - will be used to decide if unimportant data will be sacrificed for the sake of the reliability of more important data
    pub content: String,
    pub deleted: bool,
    pub hash_if_deleted: String,
    pub last_checked: i64, // with ms,
    pub reading_errors: u32,
    pub extras: String, // json
}

pub fn insert(
    pool: &Pool<SqliteConnectionManager>,
    item: &PersistentItem,
) -> Result<(), rusqlite::Error> {
    pool.get()
        .unwrap()
        .execute(
            "INSERT INTO persistentStore (
            hash,
            item_key,
            tree_hash,
            parent_hash,
            lvl,
            creator,
            created,
            importance,
            content,
            deleted,
            hash_if_deleted,
            last_checked,
            reading_errors,
            extras
        ) VALUES ( ?,?,?,?,?,?,?,?,?,?,?,?,?,? );",
            params![
                item.hash,
                item.key,
                item.tree_hash,
                item.parent_hash,
                item.lvl,
                item.creator,
                item.created,
                item.importance,
                item.content,
                item.deleted,
                item.hash_if_deleted,
                item.last_checked,
                item.reading_errors,
                item.extras
            ],
        )
        .unwrap();

    Ok(())
}

pub fn get_by_hash(
    pool: &Pool<SqliteConnectionManager>,
    hash: &String,
) -> Result<Vec<PersistentItem>, rusqlite::Error> {
    Ok(meta_get(pool, "WHERE hash = :key", hash)?)
}

pub fn get_by_key(
    pool: &Pool<SqliteConnectionManager>,
    key: &String,
) -> Result<Vec<PersistentItem>, rusqlite::Error> {
    Ok(meta_get(pool, "WHERE item_key LIKE :key", key)?)
}

pub fn get_by_tree_hash(
    pool: &Pool<SqliteConnectionManager>,
    key: &String,
) -> Result<Vec<PersistentItem>, rusqlite::Error> {
    Ok(meta_get(pool, "WHERE tree_hash = :key", key)?)
}

pub fn get_highest_by_tree_hash(
    pool: &Pool<SqliteConnectionManager>,
    key: &String,
) -> Result<Vec<PersistentItem>, rusqlite::Error> {
    Ok(meta_get(pool, "WHERE tree_hash = :key AND lvl = (SELECT MAX(lvl) FROM persistentStore WHERE tree_hash = :key)", key)?)
}

pub fn meta_get(
    pool: &Pool<SqliteConnectionManager>,
    selector: &str,
    key: &String,
) -> Result<Vec<PersistentItem>, rusqlite::Error> {
    let conn = pool.get().unwrap();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT 
            hash,
            tree_hash,
            parent_hash,
            lvl,
            creator,
            created,
            importance,
            content,
            deleted,
            hash_if_deleted,
            last_checked,
            reading_errors,
            extras,
            item_key
        FROM persistentStore {};",
            selector,
        ))
        .unwrap();
    let hash_iter = stmt.query_map_named(&[(":key", key)], |row| {
        Ok(PersistentItem {
            hash: row.get(0)?,
            tree_hash: row.get(1)?,
            parent_hash: row.get(2)?,
            lvl: row.get(3)?,
            creator: row.get(4)?,
            created: row.get(5)?,
            importance: row.get(6)?,
            content: row.get(7)?,
            deleted: row.get(8)?,
            hash_if_deleted: row.get(9)?,
            last_checked: row.get(10)?,
            reading_errors: row.get(11)?,
            extras: row.get(12)?,
            key: row.get(13)?,
        })
    })?;

    let mut vec = Vec::new();
    for hash in hash_iter {
        Some(match hash {
            Ok(e) => vec.push(e),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        });
    }

    Ok(vec)
}
