use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, propagate_version = false)]
struct Args {
    path: PathBuf
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("{:?}", args.path);

    Ok(())
}
