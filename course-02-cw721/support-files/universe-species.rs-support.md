---
main file: one file name
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Species {
    pub name: String,
    pub sapience_level: SapienceScale,
}

impl Default for Species {
    fn default() -> Self {
        Species {
            name: "Unnamed".to_string(),
            sapience_level: SapienceScale::None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[repr(u8)]
pub enum SapienceScale {
    None = 0,   // bugs
    Low = 1,    // cats, dogs
    Medium = 2, // ravens, rats, Terran humans
    High = 3,   // proper intelligent beings
}

impl SapienceScale {
    pub fn as_str(&self) -> &str {
        match self {
            SapienceScale::None => "None",
            SapienceScale::Low => "Low",
            SapienceScale::Medium => "Medium",
            SapienceScale::High => "High",
        }
    }
    pub fn as_num(&self) -> u8 {
        match self {
            SapienceScale::None => 0,
            SapienceScale::Low => 1,
            SapienceScale::Medium => 2,
            SapienceScale::High => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Species,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SapienceResponse {
    pub level: SapienceScale,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Sapient {
    pub name: String,
    pub telepathic: bool,
}

```