use std::{fs, path::PathBuf, sync::Arc};

use once_cell::sync::Lazy;
use reqwest::{header::HeaderMap, Method, Response, Url};
use tokio::sync::RwLock;

use crate::{download::err::UpdateError, CLIENT};

use self::checksumfiles::{ChecksumFiles, File};

mod checksumfiles;
pub mod err;

use anyhow::{bail, Result};

use flate2::read::MultiGzDecoder;

static BUILD_URL: Lazy<RwLock<Url>> =
    Lazy::new(|| RwLock::new(Url::parse("https://rotmg-build.decagames.com/").unwrap()));

pub async fn request_checksums(build_hash: &str, platform: &str) -> Result<ChecksumFiles> {
    let url = get_base_url(build_hash, platform, "checksum.json")?;

    let mut defheaders = HeaderMap::new();
    defheaders.append("Host", BUILD_URL.read().await.host_str().unwrap().parse()?);

    let resp = CLIENT
        .request(Method::GET, url)
        .headers(defheaders)
        .send()
        .await?;
    let resp_text = resp.text().await?;

    Ok(serde_json::from_str::<ChecksumFiles>(&resp_text)?)
}

pub async fn download_files_from_checksums(
    build_hash: &str,
    platform: &str,
    dir: &PathBuf,
    checksums_files: &Vec<File>,
    progress: Option<Arc<RwLock<f32>>>,
) -> Result<()> {
    for (i, checksum) in checksums_files.iter().enumerate() {
        let max_retries = 2;
        for n in 0..max_retries + 1 {
            let result = download_file_and_check(build_hash, platform, dir, &checksum).await;
            if result.is_ok() {
                break;
            } else if n == max_retries {
                result?;
            }
        }
        if let Some(ref progress) = progress {
            *progress.write().await = (i + 1) as f32 / checksums_files.len() as f32
        }
    }
    Ok(())
}
pub async fn download_file_and_check(
    build_hash: &str,
    platform: &str,
    dir: &PathBuf,
    file: &File,
) -> Result<()> {
    for n in 0..2 {
        if download_file(build_hash, platform, dir, &file).await? {
            break;
        } else if n == 1 {
            bail!(UpdateError(format!("Failed to download {}", file.file)));
        }
    }
    Ok(())
}
pub async fn download_file(
    build_hash: &str,
    platform: &str,
    dir: &PathBuf,
    file: &File,
) -> Result<bool> {
    let file_dir = dir.join(&file.file);
    if !file_dir.is_dir() {
        if let Some(dir) = file_dir.parent() {
            fs::create_dir_all(dir)?;
        }
    }

    let mut file_valid_flag = false;
    if let Ok(got_file) = fs::read(&file_dir) {
        if file.checksum == format!("{:x}", md5::compute(got_file)) {
            file_valid_flag = true;
        }
    };

    if !file_valid_flag {
        let compressed_file_name = format!("{}.gz", &file.file);
        let bytes = request_file(build_hash, platform, &compressed_file_name)
            .await?
            .bytes()
            .await?;

        let mut got_file = fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_dir)?;
        got_file.set_len(0)?;

        let mut decoder = MultiGzDecoder::new(&bytes[..]);

        std::io::copy(&mut decoder, &mut got_file)?;
    }

    Ok(file_valid_flag)
}

pub async fn request_file(build_hash: &str, platform: &str, file: &str) -> Result<Response> {
    let url = get_base_url(build_hash, platform, file)?;

    let mut defheaders = HeaderMap::new();
    defheaders.append("Host", BUILD_URL.read().await.host_str().unwrap().parse()?);

    let resp = CLIENT
        .request(Method::GET, url)
        .headers(defheaders)
        .send()
        .await?;
    Ok(resp)
}

fn get_base_url(build_hash: &str, platform: &str, file: &str) -> Result<Url> {
    Ok(BUILD_URL.try_read().unwrap().join(format!("build-release/{}/{}/{}", build_hash, platform, file).as_str())?)
}
