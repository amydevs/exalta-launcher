#[cfg(test)]
mod download_tests {
    use std::path::Path;
    use std::path::PathBuf;

    use exalta_core::download::*;
    use exalta_core::misc::*;

    #[tokio::test]
    async fn test_init() -> Result<(), Box<dyn std::error::Error>> {
        get_build_hash().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_checksums() -> Result<(), Box<dyn std::error::Error>> {
        let build_hash = get_build_hash().await?;
        let platform = "rotmg-exalt-win-64";
        let things = request_checksums(&build_hash, platform).await?;
        download_file(&build_hash, platform, &PathBuf::from("./help"), &things.files[0]).await?;
        Ok(())
    }

    async fn get_build_hash() -> Result<String, Box<dyn std::error::Error>> {
        Ok(init(None, None).await?.build_hash)
    }
}
