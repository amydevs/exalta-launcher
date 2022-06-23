use std::{fs, path::Path};

use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, Url, Response,
};

use crate::CLIENT;

use self::checksumfiles::{ChecksumFiles, File};

mod checksumfiles;

static BUILD_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://rotmg-build.decagames.com/").unwrap());

pub async fn request_checksums(
    build_hash: &str,
    platform: &str,
) -> Result<ChecksumFiles, Box<dyn std::error::Error>> {
    let url = get_build_url(build_hash, platform, "checksum.json")?;

    let mut defheaders = HeaderMap::new();
    defheaders.append("Host", BUILD_URL.host_str().unwrap().parse()?);

    let resp = CLIENT
        .request(Method::GET, url)
        .headers(defheaders)
        .send()
        .await?;
    let resp_text = resp.text().await?;

    Ok(serde_json::from_str::<ChecksumFiles>(&resp_text)?)
}

pub async fn download_file(
    build_hash: &str,
    platform: &str,
    dir: &Path,
    file: &File
) -> Result<(), Box<dyn std::error::Error>> {
    let file_dir = dir.join(&file.file);
    if !file_dir.is_dir() {
        if let Some(dir) = file_dir.parent() {
            fs::create_dir_all(dir)?;
        }
    }

    let mut file_valid_flag = false;
    if let Ok(got_file) = fs::read(file_dir) {
        if file.checksum == format!("{:x}", md5::compute(got_file)) {
            file_valid_flag = true;
        }
    };

    if !file_valid_flag {
        let bstream = request_file(build_hash, platform, &file.file).await?.bytes_stream();
    }

    Ok(())
}

pub async fn request_file(
    build_hash: &str,
    platform: &str,
    file: &str
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = get_build_url(build_hash, platform, file)?;

    let mut defheaders = HeaderMap::new();
    defheaders.append("Host", BUILD_URL.host_str().unwrap().parse()?);

    let resp = CLIENT
        .request(Method::GET, url)
        .headers(defheaders)
        .send()
        .await?;
    Ok(resp)
}

fn get_build_url(
    build_hash: &str,
    platform: &str,
    file: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(BUILD_URL.join(format!("build-release/{}/{}/{}", build_hash, platform, file).as_str())?)
}
