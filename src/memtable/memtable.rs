use std::collections::BTreeMap;

// MemTable is a BTreeMap of key-value pairs
pub struct MemTable { 
    table: BTreeMap<Vec<u8>, Vec<u8>>,
    size: usize,
}

// MemTable is a BTreeMap of key-value pairs
impl MemTable { 
    // new() creates a new MemTable
    pub fn new() -> Self { 
        MemTable {
            table: BTreeMap::new(),
            size: 0,
        }
    }

    // insert() inserts a key-value pair into the MemTable
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.size += key.len() + value.len();
        self.table.insert(key, value);
    }

    // get() returns the value associated with the key
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.table.get(key).cloned()
    }

    // remove() removes the key-value pair from the MemTable
    pub fn remove(&mut self, key: &[u8]) {
        // If the key exists, remove it and update the size
        if let Some(value) = self.table.remove(key) {
            self.size -= key.len() + value.len();
        }
    }

    // size() returns the size of the MemTable
    pub fn size(&self) -> usize {
        self.size
    }

    // iter() returns an iterator over the MemTable
    pub fn iter(&self) -> impl Iterator<Item = (&Vec<u8>, &Vec<u8>)> {
        self.table.iter()
    }
}