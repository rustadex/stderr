#[macro_export]
macro_rules! qlog {
    ($level:expr, $msg:expr) => {
        $crate::logger.log($level, $msg)
    };
}

#[macro_export]
macro_rules! qinfo {
    ($msg:expr) => {
        $crate::logger.log($crate::LogLevel::Info, $msg)
    };
}

#[macro_export]
macro_rules! qwarn {
    ($msg:expr) => {
        $crate::logger.log($crate::LogLevel::Warn, $msg)
    };
}

#[macro_export]
macro_rules! qerror {
    ($msg:expr) => {
        $crate::logger.log($crate::LogLevel::Error, $msg)
    };
}

#[macro_export]
macro_rules! qdebug {
    ($msg:expr) => {
        $crate::logger.log($crate::LogLevel::Debug, $msg)
    };
}

#[macro_export]
macro_rules! qtrace {
    ($msg:expr) => {
        $crate::logger.log($crate::LogLevel::Trace, $msg)
    };
}

#[macro_export]
macro_rules! qpretty {
    ($prefix:expr, $value:expr) => {
        $crate::logger.raw().print_with_prefix_debug(
            $crate::ESC::MAGENTA,
            $prefix,
            &$value
        ).ok();
    };
}
