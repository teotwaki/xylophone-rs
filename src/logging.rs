extern crate log;
extern crate time;

// Setting the exit status
use std::env;

pub struct TimeStampLogger;

impl log::Log for TimeStampLogger {
    fn enabled(&self, level: log::LogLevel, _module: &str) -> bool {
        true
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.level(), record.location().module_path) {
            println!("{} - {} - {} - {}:{} - {}",
                     now(),
                     record.level(),
                     record.location().module_path,
                     record.location().file,
                     record.location().line,
                     record.args());
        }
    }
}

fn now() -> String {
    let format = "%Y-%m-%dT%H:%M:%S%z";
    time::strftime(format,
                   &time::now()).unwrap()
}

pub fn install_logger() {
    let result = log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Trace);
        Box::new(TimeStampLogger)
    });

    match result {
        Ok(()) => { }
        Err(msg) => {
            println!("Error while setting up the logger subsystem: {}", msg);
            panic!();
        }
    };
}
