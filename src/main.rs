mod lru_cache;

fn main() {
    let lru = lru_cache::LRUCache::new(5);
    println!("{:?}", lru);
}
