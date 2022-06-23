use reqwest::Url;

use crate::CLIENT;

use self::checksumfiles::ChecksumFiles;


mod checksumfiles;

pub async fn request_checksums(build_hash: &str, platform: &str) -> Result<ChecksumFiles, Box<dyn std::error::Error>> {
    let url = get_build_url(build_hash, platform, "checksum.json")?;
    let resp = CLIENT.get(url).send().await?;
    let resp_text = resp.text().await?;
    

    Ok(serde_json::from_str::<ChecksumFiles>(&resp_text)?)
}

pub fn get_build_url(build_hash: &str, platform: &str, file: &str) -> Result<Url, Box<dyn std::error::Error>> {
    Ok(Url::parse("https://rotmg-build.decagames.com/")?.join(format!("build-release/{}/{}/{}", build_hash, platform, file).as_str())?)
}