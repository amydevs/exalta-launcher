#[cfg(test)]
mod download_tests {
    use exalta_core::misc::*;
    use exalta_core::download::*;

    #[tokio::test]
    async fn test_init() -> Result<(), Box<dyn std::error::Error>> {
        get_build_hash().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_checksums() -> Result<(), Box<dyn std::error::Error>> {
        let build_hash = get_build_hash().await?;
        let things = request_checksums(&build_hash, "rotmg-exalt-win-64").await?;
        println!("{:?}", things);
        Ok(())
    }

    async fn get_build_hash() -> Result<String, Box<dyn std::error::Error>> {
        Ok(init(None, None).await?.build_hash)
    }
}