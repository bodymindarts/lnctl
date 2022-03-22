use crate::node::hex_utils::hex_str;
use anyhow::Context;
use std::path::Path;
use tonic::metadata::*;

pub struct MacaroonData {
    metadata_value: AsciiMetadataValue,
}

impl MacaroonData {
    pub fn from_file_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let data: Vec<u8> = std::fs::read(path).context("Couldn't read macaroon file")?;
        let metadata_value = AsciiMetadataValue::try_from_bytes(hex_str(&data).as_ref())?;
        Ok(MacaroonData { metadata_value })
    }
    pub fn add_to_metadata(&self, metadata: &mut MetadataMap) {
        metadata.insert("macaroon", self.metadata_value.clone());
    }
}
