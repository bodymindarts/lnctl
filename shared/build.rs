fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "../proto/shared/shared.proto",
            "../proto/coordinator/coordinator.proto",
            "../proto/connector/connector.proto",
        ],
        &[
            "../proto/shared",
            "../proto/coordinator",
            "../proto/connector",
        ],
    )?;
    Ok(())
}
