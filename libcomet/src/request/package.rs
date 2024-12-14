use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum PackageRequest {
    Get {
        package_name: String,
        package_version: String,
    },
    Search {
        pattern: String,
    },
}
