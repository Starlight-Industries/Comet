use std::collections::HashMap;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use workspace::get_working_dir;
pub mod workspace;

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

fn main() -> Result<()> {
    // let mut foo_data: HashMap<String,PackageInfo> = HashMap::new();
    // let foo_info = PackageInfo { name: "hi".to_string()};
    // foo_data.insert("asdfasdf".to_string(), foo_info);
    
    // let test_pkg = CometPackage {test: foo_data};
    // let tomldata = toml::to_string_pretty(&test_pkg)?;
    // println!("{tomldata}");
    let path = get_working_dir()?;
    println!("{}",path.display());
    Ok(())
}
