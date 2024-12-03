mod lru_cache;

fn main() {
    let mut lru = lru_cache::LRUCache::new(5);
    lru.put("key1", String::from("1"));
    lru.put("key2", String::from("2"));
    lru.put("key3", String::from("3"));
    lru.put("key4", String::from("4"));
    lru.put("key5", String::from("5"));

    lru.print_cached_elements();

    println!("\nAdding element \"key6\"");
    lru.put("key6", String::from("6"));
    lru.print_cached_elements();

    println!("\nGetting element \"key3\"");
    lru.get("key3");
    lru.print_cached_elements();

    println!("\nUpdating element \"key5\"");
    lru.put("key5", String::from("updated value"));
    lru.print_cached_elements();
}

#[cfg(test)]
mod tests {
    use super::*;

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
