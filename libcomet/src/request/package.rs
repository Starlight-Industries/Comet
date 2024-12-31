use serde::{Deserialize, Serialize};
use versions::Versioning;

#[derive(Deserialize, Serialize,Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetRequest {
    pub package_name: String,
    pub package_version: String,
}
#[derive(Deserialize, Serialize,Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchRequest {
    pub pattern: String,
}
#[derive(Deserialize, Serialize,Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InfoRequest {
    pub package_name: String,
    pub package_version: Versioning,

}


// #[derive(Deserialize, Serialize)]
// pub enum PackageRequest {
//     Get {
//         package_name: String,
//         package_version: String,
//     },
//     Search {
//         pattern: String,
//     },
// }
