---
main file: one file name
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
/** 
 * From the https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain example code base.
 */
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}
```