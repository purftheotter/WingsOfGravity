use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex, OnceLock,
};

use log::Log;

struct StartupLogger {
    pub inner: env_logger::Logger,
    pub buffering: AtomicBool,
    pub held: Mutex<Vec<(log::Level, String, String)>>, // level, target, message
}

static LOGGER: OnceLock<StartupLogger> = OnceLock::new();

impl log::Log for StartupLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        if !self.inner.matches(record) {
            return;
        }
        if self.buffering.load(Ordering::Relaxed)
            && record.level() == log::Level::Error
            && record.target().starts_with("wgpu_hal")
        {
            self.held.lock().unwrap().push((
                record.level(),
                record.target().to_string(),
                record.args().to_string(),
            ));
        } else {
            self.inner.log(record);
        }
    }

    fn flush(&self) {
        self.inner.flush();
    }
}

pub fn init() {
    let inner = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .build();
    let max = inner.filter();

    let logger = LOGGER.get_or_init(|| StartupLogger {
        inner,
        buffering: AtomicBool::new(true),
        held: Mutex::new(Vec::new()),
    });
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(max);
}

/// Call once the adapter request has finished.
/// On success the held errors are dropped; on failure they are printed.
pub fn finish_startup(success: bool) {
    let Some(logger) = LOGGER.get() else { return };
    logger.buffering.store(false, Ordering::Relaxed);

    let held = std::mem::take(&mut *logger.held.lock().unwrap());
    if !success {
        for (level, target, message) in held {
            logger.inner.log(
                &log::Record::builder()
                    .level(level)
                    .target(&target)
                    .args(format_args!("{message}"))
                    .build(),
            );
        }
    }
}
