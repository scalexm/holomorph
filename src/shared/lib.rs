#![feature(fnbox, ip_addr, unboxed_closures, custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]

#[macro_use] extern crate log;
#[macro_use] extern crate diesel;
extern crate mio;
extern crate rustc_serialize;
extern crate crypto as rust_crypto;
extern crate eventual;
extern crate time;
extern crate protocol;

pub mod net;
pub mod chunk;
pub mod database;
pub mod config;
pub mod session;
pub mod server;
pub mod crypto;

/* helper macros for less verbosity */
#[macro_export]
macro_rules! write {
    ($server: expr, $token: expr, $data: expr) => {{
        use $crate::net::Msg;
        let tok = $token;
        let _ = $server.with(move |s| s.io_loop.send(Msg::Write(tok, $data)));
    }};
}

#[macro_export]
macro_rules! close {
    ($server: expr, $token: expr) => {{
        use $crate::net::Msg;
        let _ = $server.with(move |s| s.io_loop.send(Msg::Close($token)));
    }};
}

#[macro_export]
macro_rules! write_and_close {
    ($server: expr, $token: expr, $data: expr) => {{
        use $crate::net::Msg;
        let _ = $server.with(move |s| s.io_loop.send(Msg::WriteAndClose($token, $data)));
    }};
}

use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut, Keys, Values, Entry};
use std::hash::Hash;
use std::borrow::Borrow;

// bijective mapping between two sets, used for the shared memory thread
pub struct HashBiMap<K: Hash + Eq + Clone, V: Hash + Eq + Clone> {
    kv: HashMap<K, V>,
    vk: HashMap<V, K>,
}

impl<K: Hash + Eq + Clone, V: Hash + Eq + Clone> HashBiMap<K, V> {
    pub fn new() -> Self {
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

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> where K: Borrow<Q>, Q: Hash + Eq {
        self.kv.get(k)
    }

    pub fn inv_get<Q: ?Sized>(&self, v: &Q) -> Option<&K> where V: Borrow<Q>, Q: Hash + Eq {
        self.vk.get(v)
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool where K: Borrow<Q>, Q: Hash + Eq {
        self.kv.contains_key(k)
    }

    pub fn contains_value<Q: ?Sized>(&self, v: &Q) -> bool where V: Borrow<Q>, Q: Hash + Eq {
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
        let _ = self.kv.insert(k.clone(), v.clone());
        let _ = self.vk.insert(v, k);
        old
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V> where K: Borrow<Q>, Q: Hash + Eq {
        let v = self.kv.remove(k);
        if let Some(ref v) = v {
            let _ = self.vk.remove(&v);
        }
        v
    }

    pub fn inv_remove<Q: ?Sized>(&mut self, v: &Q) -> Option<K>
                                 where V: Borrow<Q>, Q: Hash + Eq {
        let k = self.vk.remove(v);
        if let Some(ref k) = k {
            let _ = self.kv.remove(k);
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
