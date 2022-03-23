use anyhow::Context;
use std::{fs, path::PathBuf, process};
use uuid::Uuid;

const UUID_FILE_NAME: &str = "coordinator-id";

pub fn init(path: PathBuf) -> anyhow::Result<Uuid> {
    fs::create_dir_all(&path).context("failed to create data dir")?;
    fs::write(format!("{}/pid", path.display()), process::id().to_string())?;

    let uuid_file_name = format!("{}/{}", path.display(), UUID_FILE_NAME);
    let uuid = if let Ok(content) = fs::read_to_string(&uuid_file_name) {
        if let Ok(uuid) = Uuid::parse_str(&content).context("Couldn't parse uuid") {
            uuid
        } else {
            let uuid = Uuid::new_v4();
            fs::write(uuid_file_name, uuid.to_string()).context("couldn't write uuid file")?;
            uuid
        }
    } else {
        let uuid = Uuid::new_v4();
        fs::write(uuid_file_name, uuid.to_string()).context("couldn't write uuid file")?;
        uuid
    };
    Ok(uuid)
}
