use anyhow::Context;
use std::{path::Path, process::Command};
use tonic::metadata::*;

pub struct MacaroonData {
    raw: Vec<u8>,
    metadata_value: AsciiMetadataValue,
}

impl MacaroonData {
    // pub fn from_file_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
    //     let data: Vec<u8> = std::fs::read(path).context("Couldn't read macaroon file")?;
    //     let metadata_value = BinaryMetadataValue::try_from_bytes(&data)?;
    //     Ok(MacaroonData {
    //         raw: data,
    //         metadata_value,
    //     })
    // }

    pub fn from_file_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let output = Command::new("xxd")
            .args(&["-ps", "-u", "-c", "1000"])
            .arg(path.as_ref())
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Couldn't xxd read"))
        } else {
            let mut data = output.stdout;
            println!(
                "macaroon data: {}",
                String::from_utf8(data.clone()).unwrap()
            );
            data.retain(|&z| {
                ((z >= '0' as _) && (z <= '9' as _)) | ((z >= 'A' as _) && (z <= 'F' as _))
            });
            let metadata_value = AsciiMetadataValue::try_from_bytes(&data)?;
            Ok(MacaroonData {
                raw: data,
                metadata_value,
            })
        }
    }

    pub fn add_to_metadata(&self, metadata: &mut MetadataMap) {
        metadata.insert("macaroon", self.metadata_value.clone());
    }
}
