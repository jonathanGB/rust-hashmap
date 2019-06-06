use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::mem;
use std::ops::Index;

pub struct HashMap<K, V>
where
    K: Hash + Eq,
{
    values: Vec<Option<(K, V)>>,
    len: usize,
    cap: usize,
}

pub struct HashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    hashmap: &'a HashMap<K, V>,
    at: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        const LEN: usize = 1000;
        let arr: Vec<Option<(K, V)>> = (0..LEN).into_iter().map(|_| None).collect();

        HashMap {
            values: arr,
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

        match mem::replace(&mut self.values[idx], Some((key, val))) {
            Some((_, existing_val)) => Some(existing_val),
            None => {
                self.len += 1;
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.hash(key);

        match &self.values[idx] {
            Some((_, val)) => Some(val),
            None => None,
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let idx = self.hash(key);

        match mem::replace(&mut self.values[idx], None) {
            Some((_, removed_val)) => {
                self.len -= 1;
                Some(removed_val)
            }
            None => None,
        }
    }
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.at..self.hashmap.cap {
            let pair = &self.hashmap.values[i];
            self.at += 1;

            if let Some((key, val)) = pair {
                return Some((key, val));
            }
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
            at: 0,
        }
    }
}

impl<K, Q, V> Index<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
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
