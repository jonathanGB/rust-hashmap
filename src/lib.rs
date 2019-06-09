use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::mem;
use std::ops::Index;

struct HashMapBucket<K, V>
where
    K: Hash + Eq,
{
    values: Vec<(K, V)>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        Self { values: vec![] }
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        match self.values.iter().position(|(k, _)| *k == key) {
            Some(idx) => {
                let (_, replaced_val) = mem::replace(&mut self.values[idx], (key, val));
                Some(replaced_val)
            }
            None => {
                self.values.push((key, val));
                None
            }
        }
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.values.iter().find(|(k, _)| k == key) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.values.iter().position(|(k, _)| k == key) {
            Some(idx) => {
                let (_, removed_val) = self.values.swap_remove(idx);
                Some(removed_val)
            }
            None => None,
        }
    }
}

// Implementation of a HashMap with a static number of buckets.
// Collisions for a particular bucket are stored in a list in that bucket.
// We also store length and capacity, so we technically have the necessary information
// to upgrade this hashmap to a dynamic number of buckets.
pub struct HashMap<K, V>
where
    K: Hash + Eq,
{
    buckets: Vec<HashMapBucket<K, V>>,
    len: usize,
    cap: usize,
}

pub struct HashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    hashmap: &'a HashMap<K, V>,
    at: (usize, usize),
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        const LEN: usize = 1000; // TODO: arbitrary size, could be better
        let arr: Vec<HashMapBucket<K, V>> = (0..LEN)
            .into_iter()
            .map(|_| HashMapBucket::<K, V>::new())
            .collect();

        HashMap {
            buckets: arr,
            len: 0,
            cap: LEN,
        }
    }

    fn hash<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.cap
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let idx = self.hash(&key);

        let insert_result = self.buckets[idx].insert(key, val);
        if insert_result.is_none() {
            self.len += 1;
        }

        insert_result
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.hash(key);
        self.buckets[idx].get(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.hash(key);
        let remove_result = self.buckets[idx].remove(key);
        if remove_result.is_some() {
            self.len -= 1;
        }

        remove_result
    }
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.at.0..self.hashmap.cap {
            let bucket_entries = &self.hashmap.buckets[i].values;

            for j in self.at.1..bucket_entries.len() {
                let (key, val) = &bucket_entries[j];
                self.at.1 += 1;

                return Some((key, val));
            }
            self.at.0 += 1;
            self.at.1 = 0;
        }

        None
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIter {
            hashmap: &self,
            at: (0, 0), // .0: index of buckets --- .1: index of collisions inside a bucket
        }
    }
}

impl<K, Q, V> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q> + PartialEq<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index)
            .expect("HashMap has no value linked to this key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_new() {
        HashMap::<String, u32>::new();
    }

    #[test]
    fn has_get() {
        HashMap::<String, i8>::new().get(&String::from("test"));
    }

    #[test]
    fn has_insert() {
        HashMap::new().insert(String::from("key"), String::from("val"));
    }

    #[test]
    fn inserts_has_good_return() {
        let mut map = HashMap::new();
        assert_eq!(map.insert(String::from("a"), String::from("b")), None);
        assert_eq!(
            map.insert(String::from("a"), String::from("c")),
            Some(String::from("b"))
        );
    }
}
