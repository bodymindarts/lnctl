fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "../proto/shared/shared.proto",
            "../proto/gateway/gateway.proto",
            "../proto/connector/connector.proto",
        ],
        &[
            "../proto/shared",
            "../proto/gateway",
            "../proto/connector",
        ],
    )?;
    Ok(())
}
