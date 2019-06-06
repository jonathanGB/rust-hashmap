use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::iter::Iterator;
use std::iter::IntoIterator;

pub struct HashMap {
  values: Vec<Option<(String, String)>>,
  len: usize,
  cap: usize,
}

pub struct HashMapIter<'a> {
  hashmap: &'a HashMap,
  at: usize,
}

impl HashMap {
  pub fn new() -> Self {
    const LEN: usize = 1000;
    let arr : Vec<Option<(String, String)>> = vec![None; LEN];

    HashMap{
      values: arr,
      len: 0,
      cap: LEN,
    }
  }

  fn hash(&self, key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize % self.cap
  }

  pub fn insert(&mut self, key: String, val: String) -> Option<String> {
    let idx = self.hash(&key);

    match mem::replace(&mut self.values[idx], Some((key, val))) {
      Some((_, existing_val)) => Some(existing_val),
      None => {
        self.len += 1;
        None
      }
    }
  }

  pub fn get(&self, key: &str) -> Option<&String> {
    let idx = self.hash(key);

    match &self.values[idx] {
      Some((_, val)) => Some(val),
      None => None,
    }
  }

  pub fn contains_key(&self, key: &str) -> bool {
    self.get(key).is_some()
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn remove(&mut self, key: &str) -> Option<String> {
    let idx = self.hash(key);

    match mem::replace(&mut self.values[idx], None) {
      Some((_, removed_val)) => {
        self.len -= 1;
        Some(removed_val)
      },
      None => None
    }
  }
}

impl<'a> Iterator for HashMapIter<'a> {
  type Item = (&'a String, &'a String);

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

impl<'a> IntoIterator for &'a HashMap {
  type Item = (&'a String, &'a String);
  type IntoIter = HashMapIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    HashMapIter {
      hashmap: &self,
      at: 0,
    }
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn has_new() {
    HashMap::new();
  }

  #[test]
  fn has_get() {
    HashMap::new().get(&String::from("test"));
  }

  #[test]
  fn has_insert() {
    HashMap::new().insert(String::from("key"), String::from("val"));
  }

  #[test]
  fn inserts_has_good_return() {
    let mut map = HashMap::new();
    assert_eq!(map.insert(String::from("a"), String::from("b")), None);
    assert_eq!(map.insert(String::from("a"), String::from("c")), Some(String::from("b")));
  }
}