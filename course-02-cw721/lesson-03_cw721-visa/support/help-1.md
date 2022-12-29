---
applies to: 
---

**This is a help file only**

# Answer
# `Item::save()` Example
The `impl` for `Item::save` looks like this:
```rust
pub fn save(&self, store: &mut dyn Storage, data: &T) -> StdResult<()> {}

// Used like so:
let mut deps = mock_dependencies();
let game_name: Item<String> = Item::new("game_info");
game_name.save(deps.storage, &"Race for the Galaxy".to_string())
```