use std::path::Path;

use flatc_rust;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["../proto/connector/connector.proto"], &["../proto"])?;

    flatc_rust::run(flatc_rust::Args {
        lang: "rust", // `rust` is the default, but let's be explicit
        inputs: &[Path::new("../flatbuffers/gossip.fbs")],
        out_dir: Path::new("../flatbuffers/gen/connector/"),
        ..Default::default()
    })?;

    Ok(())
}
