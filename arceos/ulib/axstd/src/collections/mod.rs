pub use alloc::collections::*;

pub struct HashMap<K, V> {
    delegate: hashbrown::HashMap<K, V>,
}

impl<K: Eq + core::hash::Hash, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self{delegate: hashbrown::HashMap::new()}
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.delegate.insert(k, v)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.delegate.get(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.delegate.get_mut(k)
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.delegate.remove(k)
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.delegate.contains_key(k)
    }

    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
        self.delegate.iter()
    }
}
