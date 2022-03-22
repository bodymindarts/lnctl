fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .compile(&["../proto/coordinator/coordinator.proto"], &["../proto"])?;
    Ok(())
}
