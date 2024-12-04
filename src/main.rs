mod lru_cache;

fn main() {
    let mut lru = lru_cache::LRUCache::new(5);
    lru.put("key1", String::from("1"));
    lru.put("key2", String::from("2"));
    lru.put("key3", String::from("3"));
    lru.put("key4", String::from("4"));
    lru.put("key5", String::from("5"));

    // Cache is at full capacity
    lru.print_cached_elements();

    // Inserting an element
    println!("\nInserting element \"key6\"");
    lru.put("key6", String::from("6"));
    lru.print_cached_elements();

    // Getting an element
    println!("\nGetting element \"key3\"");
    lru.get("key3");
    lru.print_cached_elements();

    // Updating an element
    println!("\nUpdating element \"key5\"");
    lru.put("key5", String::from("updated value"));
    lru.print_cached_elements();

    // Deleting an element, cache capacity is now at 4/5
    println!("\nDeleting element \"key2\"");
    lru.delete("key2");
    lru.print_cached_elements();

    // Cache reaches full capacity again
    println!("\nAdding element \"key7\"");
    lru.put("key7", String::from("7"));
    lru.print_cached_elements();

    // LRU element overflows the capacity and gets removed
    println!("\nAdding element \"key8\"");
    lru.put("key8", String::from("8"));
    lru.print_cached_elements();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let mut lru = lru_cache::LRUCache::new(2);
        lru.put("key1", 1);
        lru.put("key2", 2);

        assert_eq!(lru.get("key1"), Some(&1));
        assert_eq!(lru.get("key2"), Some(&2));
    }

    #[test]
    fn test_capacity_overflow() {
        let mut lru = lru_cache::LRUCache::new(2);
        lru.put("key1", 1);
        lru.put("key2", 2);
        lru.put("key3", 3);

        // "key1" should be evicted
        assert_eq!(lru.get("key1"), None);
        assert_eq!(lru.get("key2"), Some(&2));
        assert_eq!(lru.get("key3"), Some(&3));
    }

    #[test]
    fn test_lru_update_order() {
        let mut lru = lru_cache::LRUCache::new(3);
        lru.put("key1", 1);
        lru.put("key2", 2);
        lru.put("key3", 3);

        // Access "key1" to mark it as recently used
        lru.get("key1");

        // Add a new key, which should evict "key2"
        lru.put("key4", 4);

        assert_eq!(lru.get("key2"), None);
        assert_eq!(lru.get("key1"), Some(&1));
        assert_eq!(lru.get("key3"), Some(&3));
        assert_eq!(lru.get("key4"), Some(&4));
    }

    #[test]
    fn test_delete() {
        let mut lru = lru_cache::LRUCache::new(3);
        lru.put("key1", 1);
        lru.put("key2", 2);
        lru.put("key3", 3);

        // Delete "key2"
        lru.delete("key2");

        assert_eq!(lru.get("key2"), None);
        assert_eq!(lru.get("key1"), Some(&1));
        assert_eq!(lru.get("key3"), Some(&3));
    }

    #[test]
    fn test_update_existing_key() {
        let mut lru = lru_cache::LRUCache::new(2);
        lru.put("key1", String::from("value1"));
        lru.put("key1", String::from("updated_value1"));

        assert_eq!(lru.get("key1"), Some(&String::from("updated_value1")));
    }

    #[test]
    fn test_lru_cache() {
        let mut cache = lru_cache::LRUCache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Cache == [B, C, D]

        let my_value = cache.get("A");
        assert_eq!(my_value, None);
        let my_value = cache.get("D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get("B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get("C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get("X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get("B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get("D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }
}
