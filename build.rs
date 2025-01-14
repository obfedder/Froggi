use anyhow::Result;
use std::fs::{create_dir_all, File};
use tar::Builder;

fn main() -> Result<()> {
    create_dir_all("./build-tmp")?;

    let tar_file = File::create("./build-tmp/local-assets.tar")?;
    let mut tar_builder = Builder::new(tar_file);

    tar_builder.append_dir_all(".", "./src/html/web_assets")?;
    tar_builder.finish()?;

    Ok(())
}
