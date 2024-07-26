//! util run in `build.rs`
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use built;
use vergen_gitcl::{BuildBuilder, Emitter, GitclBuilder};

/// Collect info in `build.rs`
///
/// + `git`: `.git` dir path
pub fn build_gen(git: Option<String>) -> Result<(), Box<dyn Error>> {
    // 写入 OUT_DIR/pm_bin.rs
    {
        let b = include_bytes!("./build_src/pm_bin.rs");
        let p: PathBuf = [env::var("OUT_DIR").unwrap(), "pm_bin.rs".into()]
            .iter()
            .collect();
        let mut f = File::create(p)?;
        f.write_all(b)?;
    }

    // 每次编译都重新运行 `build.rs`
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(
            &GitclBuilder::default()
                .describe(true, false, None)
                .sha(false)
                .build()?,
        )?
        .emit()?;
    // `.git/index`
    if let Some(g) = git {
        let p: PathBuf = [g.clone(), ".git".into(), "index".into()].iter().collect();
        match p.canonicalize() {
            Ok(p) => {
                println!("cargo:rerun-if-changed={}", p.to_str().unwrap());
            }
            _ => {
                println!("cargo:warning=can not find {}/.git/index", g);
            }
        }
    }
    // 收集编译信息
    built::write_built_file()?;
    Ok(())
}
