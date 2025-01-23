use std::collections::HashMap;
use std::hash::Hash;

pub fn get_mut_or_default<'a, K, V>(hash_map: &'a mut HashMap<K, V>, key: &K) -> &'a mut V
where
    K: Hash + Eq + Clone,
    V: Default,
{
    if hash_map.contains_key(key) {
        hash_map.get_mut(key).unwrap()
    } else {
        hash_map.insert(key.clone(), V::default());
        hash_map.get_mut(key).unwrap()
    }
}
