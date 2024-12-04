use rust_lru_cache::LRUCache;

#[test]
fn test_integration_scenario() {
    let mut cache = LRUCache::new(3);
    cache.put("key1", 1);
    cache.put("key2", 2);
    cache.put("key3", 3);

    // Check initial state
    assert_eq!(cache.get("key1"), Some(&1));
    assert_eq!(cache.get("key2"), Some(&2));
    assert_eq!(cache.get("key3"), Some(&3));

    // Add a new element to trigger eviction
    cache.put("key4", 4);

    // key1 should be evicted
    assert_eq!(cache.get("key1"), None);
    assert_eq!(cache.get("key4"), Some(&4));

    // Access and reorder keys
    cache.get("key2");
    cache.put("key5", 5);

    // key3 should now be evicted
    assert_eq!(cache.get("key3"), None);
    assert_eq!(cache.get("key2"), Some(&2));
    assert_eq!(cache.get("key5"), Some(&5));
}
