use serde::{Deserialize, Serialize};
use crate::package::Package;

struct GetResponse {
    dependencies: Vec<Package>,
}