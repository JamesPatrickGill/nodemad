use indicatif::ProgressBar;
use reqwest::{header, Client};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};

use crate::{
    display::get_bar_style,
    utils::{download_url, install_dir, node_archive_name},
};

pub async fn fetch_node_archive(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = download_url(version).unwrap();
    let client = Client::new();

    let download_size = {
        let resp = client.head(url.as_str()).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(
                format!("Couldn't download URL: {}. Error: {:?}", url, resp.status(),).into(),
            );
        }
    };

    let request = client.get(url.as_str());

    let progress_bar = ProgressBar::new(download_size);
    progress_bar.set_style(get_bar_style());
    progress_bar.set_prefix("Downloading");
    progress_bar.set_message("");

    create_dir_all(install_dir()).await.unwrap();

    let file_install_path = format!("{}/{}", install_dir(), node_archive_name(version));
    let mut outfile = File::create(file_install_path).await?;

    let mut download = request.send().await?;

    while let Some(chunk) = download.chunk().await? {
        progress_bar.inc(chunk.len() as u64);
        outfile.write(&chunk).await?;
    }

    progress_bar.set_message("Success");
    progress_bar.finish();

    outfile.flush().await?;

    Ok(())
}
