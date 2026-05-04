use reqwest::get;
use std::fs::{self, File};
use std::path::Path;

async fn download_ubuntu_cloud_image() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://cloud-images.ubuntu.com/releases/noble/release/ubuntu-24.04-server-cloudimg-amd64.img";

    let dir = Path::new("./vm-data");
    let file_path = dir.join("cloud.img");

    // 1. Create directory if it doesn't exist
    if !dir.exists() {
        fs::create_dir_all(dir)?;
        println!("Created directory: {:?}", dir);
    }

    // 2. Skip if file already exists
    if file_path.exists() {
        println!("Image already exists at {:?}", file_path);
        return Ok(file_path.to_string_lossy().to_string());
    }

    println!("Downloading Ubuntu cloud image...");

    // 3. Download the file
    let mut response = get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Download failed: {}", response.status()).into());
    }

    // 4. Create file
    let mut file = File::create(&file_path)?;

    // 5. Write response to file
    while let Some(chunk) = response.chunk().await? {
        std::io::copy(&mut chunk.as_ref(), &mut file)?;
    }

    println!("Downloaded to {:?}", file_path);

    Ok(file_path.to_string_lossy().to_string())
}

pub fn initialize() {
    // download image
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(download_ubuntu_cloud_image()) {
        Ok(path) => println!("Image ready at: {}", path),
        Err(e) => eprintln!("Failed to download image: {}", e),
    }
}
