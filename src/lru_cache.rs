use std::collections::HashMap;

/// A cache that can hold a choosen quantity of elements,
/// deleting excess elements by order of least used.
///
/// # Type generics
/// * `K` - Key type.
/// * `V` - Value type.
pub struct LRUCache<K, V> {
    /// The cache's maximum size
    capacity: usize,
    /// HashMap containing all cached elements
    cached_elements: HashMap<K, V>,
    /// Vector containing all the keys of currently cached elements,
    /// from least to most recently used
    elements_order: Vec<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Copy + std::hash::Hash + std::fmt::Debug,
    V: std::fmt::Debug,
{
    /// Creates a new instance of LRU cache with the provided capacity.
    ///
    /// **Attributes:**
    /// * `capacity` - The cache's maximum size.
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cached_elements: HashMap::new(),
            elements_order: Vec::new(),
        }
    }

    /// Returns the element corresponding to the provided key
    /// and reorders said element as the MRU.
    ///
    /// **Attributes:**
    /// * `key` - The key of the element to reorder and return.
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

    /// If the provided key already exists in the cache, update the corresponding element's value.
    /// Otherwise, add the provided key/value pair to the cache, removing the LRU excess element in the process.
    ///
    /// In both cases, the provided element is reordered as the MRU.
    ///
    /// **Attributes:**
    /// * `key` - The key of the element to update/insert.
    /// * `value` - The value of the updated/inserted element.
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

    /// Reorders the provided key as the MRU.
    /// 
    /// **Attributes:**
    /// * `key` - The key of the element to reorder as MRU.
    fn update_order(&mut self, key: K) {
        if let Some(pos) = self.elements_order.iter().position(|k| k == &key) {
            self.elements_order.remove(pos);
            self.elements_order.push(key);
        }
    }

    /// Prints all elements currently in the cache, from least to most recently used.
    pub fn print_cached_elements(&mut self) {
        println!("---------- Currently cached values ----------");

        for key in self.elements_order.clone() {
            println!("{:?}: {:?}", key, self.cached_elements.get(&key));
        }

        println!("---------------------------------------------");
    }
}
