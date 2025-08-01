use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;

use crate::Stderr; 

static LOGGER: Lazy<Mutex<Stderr>> = Lazy::new(|| Mutex::new(Stderr::new()));

pub struct StaticLogger;

impl StaticLogger {
    pub fn info(&self, msg: &str) {
        LOGGER.lock().unwrap().info(msg);
    }

    pub fn warn(&self, msg: &str) {
        LOGGER.lock().unwrap().warn(msg);
    }

    pub fn error(&self, msg: &str) {
        LOGGER.lock().unwrap().error(msg);
    }

    // Optional: expose underlying Stderr for advanced use
    pub fn raw(&self) -> MutexGuard<'static, Stderr> {
        LOGGER.lock().unwrap()
    }
}

pub static logger: StaticLogger = StaticLogger;
