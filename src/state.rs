use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{Cleaner, CleaningTask, Room};

#[derive(Clone, Default)]
pub struct AppState {
    pub inner: Arc<Mutex<AppStateInner>>,
}

#[derive(Default)]
pub struct AppStateInner {
    pub cleaners: Vec<Cleaner>,
    pub rooms: Vec<Room>,
    pub tasks: Vec<CleaningTask>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
