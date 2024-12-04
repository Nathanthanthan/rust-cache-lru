use std::collections::HashMap;

/// A cache that can hold a choosen quantity of elements,
/// deleting excess elements by order of least used.
///
/// # Type generics
/// * `K` - Key type.
/// * `V` - Value type.
///
/// # Fields
/// * `capacity` - The cache's maximum size.
/// * `cached_elements` - HashMap containing all cached elements.
/// * `elements_order` - Vector containing all the keys of currently cached elements,
///    from least to most recently used.
#[derive(Debug)]
pub struct LRUCache<K, V> {
    /// The cache's maximum size
    capacity: usize,
    /// HashMap containing all cached elements
    cached_elements: HashMap<K, V>,
    /// Vector containing all the keys of currently cached elements,
    /// from least to most recently used
    elements_order: Vec<K>,
}

/// # General methods
impl<K, V> LRUCache<K, V>
where
    K: Eq + Copy + std::hash::Hash + std::fmt::Debug,
    V: std::fmt::Debug,
{
    /// Creates a new instance of LRU cache with the provided capacity.
    ///
    /// **Attributes:**
    /// * `capacity` - The cache's maximum size.
    ///
    /// **Exemple:**
    /// ```
    /// use rust_lru_cache::LRUCache;
    ///
    /// let mut lru = LRUCache::new(3);
    /// lru.put("key1", 1);
    /// lru.put("key2", 2);
    /// lru.put("key3", 3);
    ///
    /// // LRUCache { capacity: 5, cached_elements: {"key5": "5", "key3": "3", "key1": "1", "key4": "4", "key2": "2"}, elements_order: ["key1", "key2", "key3", "key4", "key5"] }
    /// println!("{:?}", lru);
    /// ```
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cached_elements: HashMap::new(),
            elements_order: Vec::new(),
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
    ///
    /// **Exemple:**
    /// ```
    /// use rust_lru_cache::LRUCache;
    ///
    /// let mut lru = LRUCache::new(3);
    /// lru.put("key1", 1);
    /// lru.put("key2", 2);
    /// lru.put("key3", 3);
    ///
    /// // ---------- Currently cached values ----------
    /// // "key1": Some(1)
    /// // "key2": Some(2)
    /// // "key3": Some(3)
    /// // ---------------------------------------------
    /// lru.print_cached_elements();
    /// ```
    pub fn print_cached_elements(&mut self) {
        println!("---------- Currently cached values ----------");

        for key in self.elements_order.clone() {
            println!("{:?}: {:?}", key, self.cached_elements.get(&key));
        }

        println!("---------------------------------------------");
    }
}

/// # "CRUD" methods
impl<K, V> LRUCache<K, V>
where
    K: Eq + Copy + std::hash::Hash + std::fmt::Debug,
    V: std::fmt::Debug,
{
    /// Returns the element corresponding to the provided key
    /// and reorders said element as the MRU.
    ///
    /// **Attributes:**
    /// * `key` - The key of the element to reorder and return.
    ///
    /// **Exemple:**
    /// ```
    /// use rust_lru_cache::LRUCache;
    ///
    /// let mut lru = LRUCache::new(2);
    /// lru.put("key1", 1);
    ///
    /// // "key1" exists in the cache
    /// assert_eq!(lru.get("key1"), Some(&1));
    /// // "key2" does not exist in the cache
    /// assert_eq!(lru.get("key2"), None);
    ///
    /// // Inserting "key2"
    /// lru.put("key2", 2);
    ///
    /// // "key1" exists in the cache
    /// assert_eq!(lru.get("key1"), Some(&1));
    /// // "key2" exists in the cache
    /// assert_eq!(lru.get("key2"), Some(&2));
    /// ```
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
    ///
    ///
    /// **Exemple:**
    /// ```
    /// use rust_lru_cache::LRUCache;
    ///
    /// // Capacity is 2
    /// let mut lru = LRUCache::new(2);
    ///
    /// // Inserting 3 elements
    /// lru.put("key1", 1);
    /// lru.put("key2", 2);
    /// lru.put("key3", 3);
    ///
    /// // "key1" should be evicted
    /// assert_eq!(lru.get("key1"), None);
    /// assert_eq!(lru.get("key2"), Some(&2));
    /// assert_eq!(lru.get("key3"), Some(&3));
    /// ```
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

    /// Deletes the element corresponding to the provided key from the cache.
    ///
    /// **Attributes:**
    /// * `key` - The key of the element to delete.
    ///
    /// **Exemple:**
    /// ```
    /// use rust_lru_cache::LRUCache;
    ///
    /// let mut lru = LRUCache::new(3);
    /// lru.put("key1", 1);
    /// lru.put("key2", 2);
    /// lru.put("key3", 3);
    ///
    /// // Deleting "key2"
    /// lru.delete("key2");
    ///
    /// assert_eq!(lru.get("key2"), None);
    /// assert_eq!(lru.get("key1"), Some(&1));
    /// assert_eq!(lru.get("key3"), Some(&3));
    /// ```
    pub fn delete(&mut self, key: K) {
        self.cached_elements.remove(&key);

        if let Some(pos) = self.elements_order.iter().position(|k| k == &key) {
            self.elements_order.remove(pos);
        }
    }
}
