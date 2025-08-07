//! src/lib/std.rs

//! # Stderr Functions (`stderr`)


  #[path = "stderr/stderr.rs"]
  pub mod stderr;

  #[path = "stderr/static_logger.rs"]
  pub mod static_logger;


  pub use stderr::{Stderr, StderrConfig, OptionFlag, ESC, ART };
  pub use static_logger::{ LOGGER as logger, StaticLogger };
