fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["../proto/connector/connector.proto"], &["../proto"])?;
    Ok(())
}
