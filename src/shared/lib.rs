#![feature(fnbox)]

#[macro_use]
extern crate log;
extern crate mio;
extern crate byteorder;
extern crate postgres;
extern crate rustc_serialize;

pub mod net;
pub mod io;
pub mod protocol;
pub mod pool;
pub mod database;
pub mod config;

use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut, Keys, Values, Entry};
use std::hash::Hash;
use std::borrow::Borrow;

// bijective mapping between two sets, used for the shared memory thread
pub struct HashBiMap<K: Hash + Eq + Copy, V: Hash + Eq + Copy> {
    kv: HashMap<K, V>,
    vk: HashMap<V, K>,
}

impl<K: Hash + Eq + Copy, V: Hash + Eq + Copy> HashBiMap<K, V> {
    pub fn new() -> HashBiMap<K, V> {
        HashBiMap {
            kv: HashMap::new(),
            vk: HashMap::new(),
        }
    }

    pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
        self.kv.keys()
    }

    pub fn values<'a>(&'a self) -> Values<'a, K, V> {
        self.kv.values()
    }

    pub fn iter(&self) -> Iter<K, V> {
        self.kv.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.kv.iter_mut()
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self.kv.entry(key)
    }

    pub fn len(&self) -> usize {
        self.kv.len()
    }

    pub fn is_empty(&self) -> bool {
        self.kv.is_empty()
    }

    pub fn clear(&mut self) {
        self.kv.clear();
        self.vk.clear();
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>, Q: Hash + Eq {

        self.kv.get(k)
    }

    pub fn inv_get<Q: ?Sized>(&self, v: &Q) -> Option<&K>
        where V: Borrow<Q>, Q: Hash + Eq {

        self.vk.get(v)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>, Q: Hash + Eq {

        self.kv.contains_key(k)
    }

    pub fn contains_value<Q: ?Sized>(&self, v: &Q) -> bool
        where V: Borrow<Q>, Q: Hash + Eq {

        self.vk.contains_key(v)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where K: Borrow<Q>, Q: Hash + Eq {

        self.kv.get_mut(k)
    }

    pub fn inv_get_mut<Q: ?Sized>(&mut self, v: &Q) -> Option<&mut K>
        where V: Borrow<Q>, Q: Hash + Eq {

        self.vk.get_mut(v)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old = self.remove(&k);
        let _ = self.kv.insert(k, v);
        let _ = self.vk.insert(v, k);
        old
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where K: Borrow<Q>, Q: Hash + Eq {

        let v = self.kv.remove(k);
        if let Some(v) = v {
            let _ = self.vk.remove(&v);
        }
        v
    }

    pub fn inv_remove<Q: ?Sized>(&mut self, v: &Q) -> Option<K>
        where V: Borrow<Q>, Q: Hash + Eq {

        let k = self.vk.remove(v);
        if let Some(k) = k {
            let _ = self.kv.remove(&k);
        }
        k
    }

    pub fn inverse(self) -> HashBiMap<V, K> {
        HashBiMap {
            kv: self.vk,
            vk: self.kv,
        }
    }
}
