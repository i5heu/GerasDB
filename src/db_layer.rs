use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::{process, sync::Arc};

pub struct PersistentItem {
    pub hash: String,
    pub tree_hash: String, // random hash defined on the root of the tree,
    pub parent_hash: String,
    pub lvl: u32, //the level above the tree root,
    pub creator: String, // the creator of this link for diff between sys and usr,
    pub created: i64, // with ms,
    pub importance: u32, // the importance of the data - higher is more important - will be used to decide if unimportant data will be sacrificed for the sake of the reliability of more important data
    pub content: String,
    pub deleted: bool,
    pub hash_if_deleted: String,
    pub last_checked: i64, // with ms,
    pub reading_errors: u32,
    pub extras: String, // json
}


pub fn insert(conn: &Arc<Connection>, item: &PersistentItem) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO persistentStore (
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
            extras
        ) VALUES (
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?,
        ?
      );",
      params![
        item.hash,
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
      ] 
    )?;


    Ok(())
}


pub fn get(conn: &Arc<Connection>, hash: &String) -> Result<PersistentItem, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT 
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
            extras
        FROM persistentStore WHERE hash = :search_hash")?;

    let hash_iter = stmt.query_map(params![hash], |row| Ok(PersistentItem { 
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
    }))?;


    let mut foo:Option<PersistentItem> = None;

    for hash in hash_iter {
        foo = Some( match hash {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        });
    }
    
    if let Some(x) = foo {
        Ok(x)
    } else {
        Ok(PersistentItem{
            hash: String::from("NOT FOUND"),
            tree_hash: String::from(""),
            parent_hash: String::from(""),
            hash_if_deleted: String::from(""),
            lvl: 0,
            creator: String::from(""),
            created: 0,
            importance: 0,
            content: String::from(""),
            deleted: false,
            last_checked: 0,
            reading_errors: 0,
            extras: String::from(""),
        })
    }
}