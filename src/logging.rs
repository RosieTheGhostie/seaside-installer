use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
}

pub static mut LOG_LEVEL: LogLevel = LogLevel::Info;

#[macro_export]
macro_rules! debug {
    () => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Debug {
            ::minimal_logging::macros::debugln!();
        }
    };
    ($($arg:tt)*) => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Debug {
            ::minimal_logging::macros::debugln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! info {
    () => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Info {
            ::minimal_logging::macros::grayln!();
        }
    };
    ($($arg:tt)*) => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Info {
            ::minimal_logging::macros::grayln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    () => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Warn {
            ::minimal_logging::macros::warnln!();
        }
    };
    ($($arg:tt)*) => {
        if unsafe { $crate::logging::LOG_LEVEL } <= $crate::logging::LogLevel::Warn {
            ::minimal_logging::macros::warnln!($($arg)*);
        }
    };
}
