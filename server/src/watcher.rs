use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tracing::{info, error};

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn new<F>(path: PathBuf, callback: F) -> Self
    where
        F: Fn() + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        ).unwrap();
        
        if let Err(e) = watcher.watch(&path, RecursiveMode::Recursive) {
            error!("Failed to watch path {:?}: {}", path, e);
        } else {
            info!("Watching path {:?} for changes", path);
        }
        
        // Spawn a task to handle file changes
        std::thread::spawn(move || {
            for event in rx {
                if let notify::EventKind::Create(_) | notify::EventKind::Modify(_) = event.kind {
                    info!("File change detected: {:?}", event.paths);
                    callback();
                }
            }
        });
        
        Self { _watcher: watcher }
    }
}
