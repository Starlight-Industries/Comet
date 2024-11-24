use serde::{Deserialize, Serialize};

type Semversion = (u8,u8,u8);
/// # DO NOT CHANGE
#[derive(Debug,Clone, Copy,Serialize,Deserialize)]
pub struct CometConfig {
    schema_version: Semversion
}


impl Default for CometConfig {
    fn default() -> Self {
        Self { schema_version: (0u8,1u8,0u8)  }
    }
}