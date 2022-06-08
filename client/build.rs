use std::path::PathBuf;

use near_abi_rs::Config;

fn main() -> anyhow::Result<()> {
    let config = Config {
        out_dir: Some(PathBuf::from("gen")),
    };
    config.compile_abi(&["../target/near/adder/abi.json"])?;
    Ok(())
}
