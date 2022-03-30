use std::collections::HashMap;
use std::hash::Hash;

pub struct NyahUtilIndexedMap<K: Hash, V> {
    pub key_map: HashMap<K, usize>,
    pub values: Vec<V>,
}
