use std::collections::HashMap;

#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<String, (String, usize)>,
    order: Vec<String>,
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
