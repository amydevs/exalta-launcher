use once_cell::sync::Lazy;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method, Url,
};

use crate::CLIENT;

use self::checksumfiles::ChecksumFiles;

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

pub fn get_build_url(
    build_hash: &str,
    platform: &str,
    file: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(BUILD_URL.join(format!("build-release/{}/{}/{}", build_hash, platform, file).as_str())?)
}
