use std::fmt::{ Debug, Display };
use std::io;
use termcolor::{Color};
use crate::stderr::{Stderr, OptionFlag, ESC, ART};

pub enum LogLevel {
  Okay,
  Info,
  Note,
  Warn,
  Error,
  Debug,
  Trace,
  Magic,
  Silly,
  DevLog
}


pub struct DebugPrinter<'a> {
    inner: &'a mut Stderr,
}

impl<'a> DebugPrinter<'a> {

    pub fn warn<T: Debug>(&mut self, value: &T) {
        self.inner.warn_debug(value);
    }

    pub fn info<T: Debug>(&mut self, value: &T) {
        self.inner.info_debug(value);
    }

    pub fn error<T: Debug>(&mut self, value: &T) {
        self.inner.error_debug(value);
    }

    pub fn trace<T: Debug>(&mut self, value: &T) {
        self.inner.trace_debug(value);
    }

    pub fn silly<T: Debug>(&mut self, value: &T) {
        self.inner.silly_debug(value);
    }

    pub fn magic<T: Debug>(&mut self, value: &T) {
        self.inner.magic_debug(value);
    }

    pub fn okay<T: Debug>(&mut self, value: &T) {
        self.inner.okay_debug(value);
    }

    pub fn note<T: Debug>(&mut self, value: &T) {
        self.inner.note_debug(value);
    }

    pub fn devlog<T: Debug>(&mut self, value: &T) {
        self.inner.devlog_debug(value);
    }

    pub fn debug<T: Debug>(&mut self, value: &T) {
        self.inner.debug_debug(value);
    }

}


impl Stderr {

  pub fn inspect(&mut self) -> DebugPrinter {
      DebugPrinter { inner: self }
  }

  pub fn log(&mut self, level: LogLevel, msg: &str) {
    let (color, symbol) = match level {
        LogLevel::Okay  => (ESC::GREEN,   ART::Pass),
        LogLevel::Warn  => (ESC::ORANGE,  ART::Delta),
        LogLevel::Error => (ESC::RED,     ART::Fail),
        LogLevel::Info  => (ESC::BLUE,    ART::Lambda),
        LogLevel::Note  => (ESC::BLUE,    ART::Lambda),
        LogLevel::Debug => (ESC::CYAN,    ART::Boto),
        LogLevel::Trace => (ESC::GREY,    ART::Dots),
        LogLevel::Magic => (ESC::PURPLE,  ART::Bolt),
        LogLevel::Silly => (ESC::MAGENTA, ART::Phi),
        LogLevel::DevLog => (ESC::MAGENTA, ART::Boto),
      };

    let _ = self.print_with_prefix(color, symbol, msg);
  }


  pub fn print_with_prefix_debug<T: std::fmt::Debug>(
      &mut self,
      color: Color,
      prefix: impl Display,
      value: &T,
  ) -> io::Result<()> {
      if self.check_flag(OptionFlag::Quiet){ return Ok(()); }

      self.set_fg(color)?;

      let formatted_prefix = match &self.label {
          Some(label) => format!("[{}][{}]", label, prefix),
          None => format!("[{}]", prefix),
      };

      writeln!(&mut self.writer, "{} {:#?}", formatted_prefix, value)?;
      self.writer.reset()
  }


  /// Pretty-print an error struct (always displayed).
  pub fn error_debug<T: Debug>(&mut self, value: &T) {
      let _ = self.print_with_prefix_debug(ESC::RED, ART::Fail, value);
  }

  /// Pretty-print a warning struct (quiet suppressible).
  pub fn warn_debug<T: Debug>(&mut self, value: &T) {
      let _ = self.print_with_prefix_debug(ESC::ORANGE, ART::Delta, value);
  }

  /// Pretty-print an info struct (quiet suppressible).
  pub fn info_debug<T: Debug>(&mut self, value: &T) {
      let _ = self.print_with_prefix_debug(ESC::BLUE, ART::Lambda, value);
  }

  /// Pretty-print an okay struct (quiet suppressible).
  pub fn okay_debug<T: Debug>(&mut self, value: &T) {
      let _ = self.print_with_prefix_debug(ESC::GREEN, ART::Pass, value);
  }

  /// Pretty-print a note struct (quiet suppressible).
  pub fn note_debug<T: Debug>(&mut self, value: &T) {
      let _ = self.print_with_prefix_debug(ESC::BLUE, ART::Right, value);
  }

  /// Pretty-print a debug struct (only shown in debug mode).
  pub fn debug_debug<T: Debug>(&mut self, value: &T) {
      if !self.config.debug { return; }
      let _ = self.print_with_prefix_debug(ESC::CYAN, ART::Boto, value);
  }

  /// Pretty-print a devlog struct (only shown in dev mode).
  pub fn devlog_debug<T: Debug>(&mut self, value: &T) {
      if !self.config.dev { return; }
      let _ = self.print_with_prefix_debug(ESC::RED2, ART::Boto, value);
  }

  /// Pretty-print a trace struct (only shown in trace mode).
  pub fn trace_debug<T: Debug>(&mut self, value: &T) {
      if !self.config.trace { return; }
      let _ = self.print_with_prefix_debug(ESC::GREY, ART::Dots, value);
  }

  /// Pretty-print a magic struct (only shown in silly mode).
  pub fn magic_debug<T: Debug>(&mut self, value: &T) {
      if !self.config.silly { return; }
      let _ = self.print_with_prefix_debug(ESC::PURPLE, ART::Bolt, value);
  }

  /// Pretty-print a silly struct (only shown in silly mode).
  pub fn silly_debug<T: Debug>(&mut self, value: &T) {
      if !self.config.silly { return; }
      let _ = self.print_with_prefix_debug(ESC::MAGENTA, ART::Phi, value);
  }
}
