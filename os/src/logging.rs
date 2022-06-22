use core::fmt;

use lazy_static::lazy_static;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

// SimpleLogger 数据结构，只包括了 LOG_LEVEL
struct SimpleLogger{
    level_filter: LevelFilter
}
impl SimpleLogger {
    fn new() -> Self {
        SimpleLogger{
            level_filter: match option_env!("LOG") {
                Some("error") => LevelFilter::Error,
                Some("warn") => LevelFilter::Warn,
                Some("info") => LevelFilter::Info,
                Some("debug") => LevelFilter::Debug,
                Some("trace") => LevelFilter::Trace,
                _ => LevelFilter::Trace,
            }
        }
    }
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
// 在字符串中加入对应的颜色标记
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {
        {
            format_args!("\x1b[{}m{}\x1b[0m", $color_code as u8, $args)
        }
    };
}
fn print_in_color(args: fmt::Arguments, color_code: u8) {
    crate::console::print(with_color!(args, color_code));
}
impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        print_in_color(
            format_args!(
                "[{:>5}] {}\n",
                record.level(),
                record.args()
            ),
            level_to_color_code(record.level()),
        );
    }
    fn flush(&self) {}
}

lazy_static!{
    static ref LOGGER: SimpleLogger = SimpleLogger::new();   
}
  
pub fn init() {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(LOGGER.level_filter);
}