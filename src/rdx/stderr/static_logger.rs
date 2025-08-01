use std::sync::{Mutex, MutexGuard};
use once_cell::sync::Lazy;

use crate::Stderr; 

static LOGGER_: Lazy<Mutex<Stderr>> = Lazy::new(|| Mutex::new(Stderr::new()));

pub struct StaticLogger;

impl StaticLogger {
    pub fn info(&self, msg: &str) {
        LOGGER_.lock().unwrap().info(msg);
    }

    pub fn warn(&self, msg: &str) {
        LOGGER_.lock().unwrap().warn(msg);
    }

    pub fn error(&self, msg: &str) {
        LOGGER_.lock().unwrap().error(msg);
    }

    pub fn okay(&self, msg: &str) {
        LOGGER_.lock().unwrap().okay(msg);
    }


    // Optional: expose underlying Stderr for advanced use
    pub fn raw(&self) -> MutexGuard<'static, Stderr> {
        LOGGER_.lock().unwrap()
    }
}

pub static LOGGER: StaticLogger = StaticLogger;
