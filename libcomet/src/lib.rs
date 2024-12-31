#![feature(error_generic_member_access)]
pub mod package;
pub mod repo;
pub mod request;
pub mod response;
pub mod workspace;

pub mod prelude {
    #[allow(unused_imports)]
    use crate::package::*;
}
