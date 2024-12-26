use serde::{Deserialize, Serialize};

use crate::package::Architecture;

#[derive(Debug, Deserialize, Serialize)]
pub struct IdentityRequest {
    pub uid: String,
    pub display_name: String,
    pub architectures: Vec<Architecture>,
    pub legacy_support: bool,
}
