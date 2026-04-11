//! Shared message types for background task communication.

use std::path::PathBuf;

use crate::apkpure::ApkVersion;

#[derive(Debug, Clone)]
pub enum TaskMsg {
    Log(String),
    /// `(bytes_done, total_bytes)` — zero total means indeterminate.
    Progress(u64, u64),
    VersionList(Vec<ApkVersion>),
    Downloaded(PathBuf),
    PemKeys(Vec<String>),
    Done,
    Error(String),
}

pub type Sender = std::sync::mpsc::Sender<TaskMsg>;
pub type Receiver = std::sync::mpsc::Receiver<TaskMsg>;

pub fn channel() -> (Sender, Receiver) {
    std::sync::mpsc::channel()
}
