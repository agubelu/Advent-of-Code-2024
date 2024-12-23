#![allow(dead_code)]
use rustc_hash::FxHashMap;
use num_traits::int::PrimInt;
use std::hash::Hash;

pub struct IDAssigner<K: Hash + Eq + Clone, V: PrimInt + Hash + Eq> {
    data: FxHashMap<K, V>,
    rev: FxHashMap<V, K>,
}

impl<K: Hash + Eq + Clone, V: PrimInt + Hash + Eq> IDAssigner<K, V> {
    pub fn new() -> Self {
        Self { data: FxHashMap::default(), rev: FxHashMap::default() }
    }

    pub fn get_id(&mut self, elem: K) -> V {
        if let Some(id) = self.data.get(&elem) {
            *id
        } else {
            let next_id = V::from(self.data.len()).unwrap();
            self.rev.insert(next_id, elem.clone());
            self.data.insert(elem, next_id);
            next_id
        }
    }

    pub fn get_elem(&self, id: V) -> Option<&K> {
        self.rev.get(&id)
    }
}
