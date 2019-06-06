use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

pub struct HashMap {
  pub values: Vec<Option<String>>,
  len: usize,
  cap: usize,
}

impl HashMap {
  pub fn new() -> Self {
    const LEN: usize = 1000;
    let arr : Vec<Option<String>> = vec![None; LEN];
    HashMap{values: arr, len: 0, cap: LEN}
  }

  fn hash(&self, key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize % self.cap
  }

  pub fn insert(&mut self, key: String, val: String) -> Option<String> {
    let idx = self.hash(&key);

    let existing_val = mem::replace(&mut self.values[idx], Some(val));
    if let None = existing_val {
      self.len += 1;
    }

    existing_val
  }

  pub fn get(&mut self, key: &str) -> &Option<String> {
    let idx = self.hash(key);
    &self.values[idx]
  }

  pub fn contains_key(&mut self, key: &str) -> bool {
    self.get(key).is_some()
  }

  pub fn len(&self) -> usize {
    self.len
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