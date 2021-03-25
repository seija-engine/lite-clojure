use std::hash::Hash;
use fnv::FnvHashMap;
pub struct ScopedMap<K: Eq + Hash, V> {
    map: FnvHashMap<K, Vec<V>>,
    scopes: Vec<Option<K>>,
}

impl<K: Eq + Hash, V> Default for ScopedMap<K, V> {
    fn default() -> Self {
        ScopedMap {
            map: FnvHashMap::default(),
            scopes: Vec::default(),
        }
    }
}

impl<K: Eq + Hash + Clone, V> ScopedMap<K, V> {
    pub fn new() -> ScopedMap<K, V> {
        ScopedMap::default()
    }

}