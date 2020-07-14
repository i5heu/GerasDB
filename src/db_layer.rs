pub struct PersistentItem {
    hash: SHA3,
    treeHash: SHA3, // random hash defined on the root of the tree,
    parentHash: SHA3,
    lvl: int, //the level above the tree root,
    creator: SHA3, // the creator of this link for diff between sys and usr,
    created: timestamp, // with ms,
    importance: int, // the importance of the data - higher is more important - will be used to decide if unimportant data will be sacrificed for the sake of the reliability of more important data
    content: blob,
    deleted: bool,
    hashIfDeleted: SHA3,


    lastChecked: timestamp, // with ms,
    readingErrors: int,
    extras: string, // json
}
