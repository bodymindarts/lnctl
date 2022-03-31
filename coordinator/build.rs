use std::path::Path;

use flatc_rust;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "../proto/coordinator/coordinator.proto",
            "../proto/connector/connector.proto",
        ],
        &["../proto"],
    )?;

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[
            Path::new("../flatbuffers/shared.fbs"),
            Path::new("../flatbuffers/channels_archive.fbs"),
        ],
        out_dir: Path::new("../flatbuffers/gen/coordinator/"),
        ..Default::default()
    })?;
    Ok(())
}
