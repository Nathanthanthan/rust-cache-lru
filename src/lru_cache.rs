use std::collections::HashMap;

#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, (i32, usize)>,
    order: Vec<i32>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }
}
