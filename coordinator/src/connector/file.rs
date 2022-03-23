use notify::{watcher, RecursiveMode, Watcher};
use std::{
    fs,
    sync::mpsc::channel as std_channel,
    {path::PathBuf, time::Duration},
};
use tokio::sync::mpsc;

const DEBOUNCE_DURATION: Duration = Duration::from_secs(2);

pub async fn watch(connectors_file: PathBuf) -> anyhow::Result<mpsc::Receiver<Vec<String>>> {
    let (tx, mut rx) = mpsc::channel(50);
    let (send, receive) = std_channel();
    let mut watcher = watcher(send, DEBOUNCE_DURATION).unwrap();
    println!("Monitoring {} for changes", connectors_file.display());
    watcher.watch(connectors_file.clone(), RecursiveMode::NonRecursive)?;
    let unwatch_connectors_file = connectors_file.clone();
    tokio::task::spawn_blocking(move || {
        loop {
            match receive.recv() {
                Ok(event) => {
                    let sender = tx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = sender.send(event).await {
                            eprintln!("Error watching file: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error watching file: {}", e);
                    break;
                }
            }
        }
        let _ = watcher.unwatch(unwatch_connectors_file);
    });
    let (list_sender, list_receiver) = mpsc::channel(50);
    let spawned_list_sender = list_sender.clone();
    let absolute_connectors_file = fs::canonicalize(&connectors_file)?;
    tokio::spawn(async move {
        use notify::DebouncedEvent::*;
        while let Some(event) = rx.recv().await {
            match event {
                Write(path) | Create(path) | Remove(path) | Rename(_, path)
                    if path == absolute_connectors_file =>
                {
                    println!("Reloading connectors file...");
                    match read_connectors_file(absolute_connectors_file.clone()) {
                        Ok(content) => {
                            if let Err(e) = spawned_list_sender.send(content).await {
                                eprintln!("Error sending connectors list: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading connectors file: {}", e);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    });
    if let Ok(content) = read_connectors_file(connectors_file) {
        let _ = list_sender.send(content).await;
    }
    Ok(list_receiver)
}

fn read_connectors_file(connectors_file: PathBuf) -> anyhow::Result<Vec<String>> {
    match fs::read_to_string(connectors_file) {
        Ok(content) => Ok(content.lines().map(|s| s.to_string()).collect()),
        Err(e) => Err(anyhow::anyhow!("Error reading connectors file: {}", e)),
    }
}
