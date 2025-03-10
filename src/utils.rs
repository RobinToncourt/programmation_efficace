use std::cmp::Ord;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::{Hash, BuildHasher};

/// # Panics
///
/// Can't panic.
pub fn get_mut_or_default_hash_map<'a, K, V, S>(hash_map: &'a mut HashMap<K, V, S>, key: &K) -> &'a mut V
where
    K: Hash + Eq + Clone,
    V: Default,
    S: BuildHasher,
{
    if hash_map.contains_key(key) {
        hash_map.get_mut(key).unwrap()
    } else {
        hash_map.insert(key.clone(), V::default());
        hash_map.get_mut(key).unwrap()
    }
}

/// # Panics
///
/// Can't panic.
pub fn get_mut_or_default_btree_map<'a, K, V>(
    btree_map: &'a mut BTreeMap<K, V>,
    key: &K,
) -> &'a mut V
where
    K: Hash + Ord + Clone,
    V: Default,
{
    if btree_map.contains_key(key) {
        btree_map.get_mut(key).unwrap()
    } else {
        btree_map.insert(key.clone(), V::default());
        btree_map.get_mut(key).unwrap()
    }
}
