use std::collections::HashMap;

pub struct LRUCache<K, V> {
    capacity: usize,
    cached_elements: HashMap<K, V>,
    elements_order: Vec<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Copy + std::hash::Hash + std::fmt::Display,
    V: std::fmt::Debug,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cached_elements: HashMap::new(),
            elements_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if self.cached_elements.contains_key(&key) {
            self.update_order(key);

            match self.cached_elements.get(&key) {
                Some(v) => Some(v),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.cached_elements.contains_key(&key) {
            self.cached_elements.insert(key.clone(), value);
            self.update_order(key);
        } else {
            if self.cached_elements.len() == self.capacity {
                let lru_key = self.elements_order.remove(0);
                self.cached_elements.remove(&lru_key);
            }

            self.cached_elements.insert(key.clone(), value);
            self.elements_order.push(key);
        }
    }

    fn update_order(&mut self, key: K) {
        if let Some(pos) = self.elements_order.iter().position(|k| k == &key) {
            self.elements_order.remove(pos);
            self.elements_order.push(key);
        }
    }

    pub fn print_cache_elements(&mut self) {
        println!("---------- Currently cached values ----------");

        for key in self.elements_order.clone() {
            println!("{}: {:?}", key, self.cached_elements.get(&key));
        }

        println!("---------------------------------------------");
    }
}
