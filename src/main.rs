use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use more_wallpapers::Mode;
use rand::{distr::Alphanumeric, Rng};
use reqwest::Client;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, propagate_version = false)]
struct Args {
    url: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let temp_file_name: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let wallpaper_path = env::temp_dir().join(temp_file_name);

    download_file(
        &Client::new(),
        &args.url,
        &wallpaper_path,
        "Downloading wallpaper..."
    ).await?;

    println!("Settings wallpaper...");
    let wallpaper = wallpaper_path.to_str().unwrap();
    let wallpapers = vec![wallpaper];
    more_wallpapers::set_wallpapers_from_vec(wallpapers, wallpaper, Mode::Crop)?;

    print!("Done.");
    Ok(())
}

async fn download_file(
    client: &Client,
    url: &str,
    path: &PathBuf,
    message: &'static str,
) -> Result<()> {
    // Reqwest setup
    let response = client.get(url).send().await?;
    let total_size = response.content_length().unwrap_or(0);

    // Indicatif setup
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_message(message);

    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));

    // Download file
    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;
        progress_bar.set_position(downloaded);
    }

    progress_bar.finish();
    Ok(())
}