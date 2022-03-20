use anyhow::Context;
use std::path::Path;
use tonic::metadata::*;

pub struct MacaroonData {
    raw: Vec<u8>,
    metadata_value: BinaryMetadataValue,
}

impl MacaroonData {
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let data: Vec<u8> = std::fs::read(path).context("Couldn't read macaroon file")?;
        let metadata_value = BinaryMetadataValue::try_from_bytes(&data)?;
        Ok(MacaroonData {
            raw: data,
            metadata_value,
        })
    }

    pub fn add_to_metadata(&self, metadata: &mut MetadataMap) {
        metadata.insert_bin("macaroon", self.metadata_value.clone());
    }
}
