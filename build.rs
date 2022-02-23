fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute("Peer", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "GetNodeStatusResponse",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["proto/lnctl.proto"], &["proto"])?;
    Ok(())
}
