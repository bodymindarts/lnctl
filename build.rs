fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/lnctl.proto"], &["proto"])?;
    tonic_build::configure().compile(&["proto/lnd/lightning.proto"], &["proto"])?;
    Ok(())
}
