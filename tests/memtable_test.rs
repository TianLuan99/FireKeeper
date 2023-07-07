extern crate firekeeper;

use firekeeper::memtable::MemTable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut memtable = MemTable::new();
        memtable.insert(vec![1], vec![2]);
        assert_eq!(memtable.size(), 2);
    }

    #[test]
    fn test_get() {
        let mut memtable = MemTable::new();
        memtable.insert(vec![1], vec![2]);
        assert_eq!(memtable.get(&vec![1]), Some(vec![2]));
    }

    #[test]
    fn test_remove() {
        let mut memtable = MemTable::new();
        memtable.insert(vec![1], vec![2]);
        memtable.remove(&vec![1]);
        assert_eq!(memtable.size(), 0);
    }

    #[test]
    fn test_size() {
        let mut memtable = MemTable::new();
        memtable.insert(vec![1], vec![2]);
        assert_eq!(memtable.size(), 2);
    }
}