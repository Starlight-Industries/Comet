use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct CometPackage {
    test: HashMap<String,PackageInfo>
}
#[derive(Debug,Serialize,Deserialize)]
struct PackageInfo {
    name: String
}
#[derive(Debug,Serialize,Deserialize)]
enum PackageState {
    Stable,
    Dev,
    Beta,
    Source,
}
/*
stable-multiarch-mono
x86_64-stable-
*/
#[derive(Debug,Serialize,Deserialize)]
pub enum Bitness {
    Bits64,
    Bits32,
}
#[derive(Debug,Serialize,Deserialize)]
pub enum Architecture   {
    Amd64(String/*Version: x86 v1,v2,v3,etc */,Bitness),
    PowerPC(Bitness),
    Aarch64(Bitness),
    Arm(Bitness),
    Mips(Bitness),
    RiscV(Bitness),
}
#[derive(Debug,Serialize,Deserialize)]
enum RepoType {
    Mono,
    Mirror,
    Distributed,
    Other,
}

type RepositoryInfo = (PackageState,Architecture,RepoType);
/* Stable-Amd64-Mono */
#[derive(Debug,Serialize,Deserialize)]
struct Cometrepository {
    name: String,
    info: RepositoryInfo,
}