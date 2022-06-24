#[cfg(test)]
mod download_tests {
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
        let mut things = request_checksums(&build_hash, platform).await?;

        let spwn = tokio::spawn(async move {
            things.files.truncate(2);
            download_files_from_checksums(
                &build_hash,
                platform,
                &PathBuf::from("./help"),
                &things.files,
                None,
            )
            .await
            .unwrap();
        });
        spwn.await?;
        Ok(())
    }

    async fn get_build_hash() -> Result<String, Box<dyn std::error::Error>> {
        Ok(init(None, None).await?.build_hash)
    }
}
