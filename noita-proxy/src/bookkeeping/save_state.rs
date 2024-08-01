use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};

use tracing::{error, info, warn};

pub trait SaveStateEntry: bitcode::Encode + bitcode::DecodeOwned {
    const FILENAME: &'static str;
}

struct SaveStateInner {
    game_started: AtomicBool,
}

/// Allows persisting extra run state (like chunks). Cleared between runs.
#[derive(Clone)]
pub struct SaveState {
    inner: Arc<SaveStateInner>,
    path: PathBuf,
    has_savestate: bool,
}

impl SaveState {
    pub(crate) fn new(path: PathBuf) -> Self {
        let has_savestate = path.join("run_info.bit").exists();
        info!("Has savestate: {has_savestate}");
        if let Err(err) = fs::create_dir_all(&path) {
            error!("Error while creating directories: {err}");
        }
        let path = path.canonicalize().unwrap_or(path);
        info!("Will save to: {}", path.display());
        Self {
            path,
            inner: Arc::new(SaveStateInner {
                game_started: false.into(),
            }),
            has_savestate,
        }
    }

    pub(crate) fn save<D: SaveStateEntry>(&self, data: &D) {
        if !self.inner.game_started.load(atomic::Ordering::SeqCst) {
            info!("Skipping save of {}, game not started yet", D::FILENAME);
            return;
        }

        let path = self.path_for_filename(D::FILENAME);
        let encoded = bitcode::encode(data);
        if let Err(err) = fs::write(&path, encoded) {
            error!("Error while saving to {:?}: {err}", D::FILENAME);
        }
        info!("Saved {}", path.display());
    }

    pub(crate) fn load<D: SaveStateEntry>(&self) -> Option<D> {
        let path = self.path_for_filename(D::FILENAME);
        let data = fs::read(&path)
            .inspect_err(|err| warn!("Could not read {:?}: {err}", D::FILENAME))
            .ok()?;
        bitcode::decode(&data)
            .inspect_err(|err| error!("Could not decode {:?}: {err}", D::FILENAME))
            .ok()
    }

    pub(crate) fn mark_game_started(&self) {
        self.inner
            .game_started
            .store(true, atomic::Ordering::SeqCst);
    }

    pub(crate) fn reset(&self) {
        fs::remove_dir_all(&self.path).ok();
        fs::create_dir_all(&self.path).ok();
    }

    /// true if had a savestate initially.
    pub(crate) fn has_savestate(&self) -> bool {
        self.has_savestate
    }

    fn path_for_filename(&self, filename: &str) -> PathBuf {
        self.path.join(format!("{filename}.bit"))
    }
}
