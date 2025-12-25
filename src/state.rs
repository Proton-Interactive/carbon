use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SyncCommand {
    Import,
    Export,
    Sourcemap,
}

#[derive(Clone)]
pub struct AppState {
    pub pending_command: Arc<Mutex<Option<SyncCommand>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pending_command: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_command(&self, command: SyncCommand) {
        let mut lock = self.pending_command.lock().expect("Failed to lock mutex");
        *lock = Some(command);
    }

    pub fn pop_command(&self) -> Option<SyncCommand> {
        let mut lock = self.pending_command.lock().expect("Failed to lock mutex");
        lock.take()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
