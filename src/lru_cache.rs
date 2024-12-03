use std::collections::HashMap;

pub struct LRUCache {
    capacity: usize,
    cached_elements: HashMap<String, String>,
    elements_order: Vec<String>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cached_elements: HashMap::new(),
            elements_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        let key_string = key.to_string();

        if self.cached_elements.contains_key(&key_string) {
            self.update_order(key_string);

            match self.cached_elements.get(key) {
                Some(v) => Some(v),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn put(&mut self, key: &str, value: String) {
        let key_string = key.to_string();

        if self.cached_elements.contains_key(&key_string) {
            self.cached_elements.insert(key_string.clone(), value);
            self.update_order(key_string);
        } else {
            if self.cached_elements.len() >= self.capacity {
                let lru_key = self.elements_order.remove(0);
                self.cached_elements.remove(&lru_key);
            }

            self.cached_elements.insert(key_string.clone(), value);
            self.elements_order.push(key_string);
        }
    }

    fn update_order(&mut self, key: String) {
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
