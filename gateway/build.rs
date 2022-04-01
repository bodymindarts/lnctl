use std::path::Path;

use flatc_rust;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[
            Path::new("../flatbuffers/shared.fbs"),
            Path::new("../flatbuffers/channels_archive.fbs"),
        ],
        out_dir: Path::new("../flatbuffers/gen/gateway/"),
        ..Default::default()
    })?;
    Ok(())
}
