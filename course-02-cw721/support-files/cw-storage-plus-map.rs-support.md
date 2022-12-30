---
main file: cw-storage-plus/map.rs 
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
impl<'a, K, T> Map<'a, K, T>
where
    T: Serialize + DeserializeOwned,
    K: PrimaryKey<'a>,
{
  pub fn save(&self, store: &mut dyn Storage, k: K, data: &T) -> StdResult<()> {
      self.key(k).save(store, data)
  }
  /// load will return an error if no data is set at the given key, or on parse error
  pub fn load(&self, store: &dyn Storage, k: K) -> StdResult<T> {
      self.key(k).load(store)
  }
}
```