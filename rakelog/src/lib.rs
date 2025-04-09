pub mod rake_log {
    use chrono::SecondsFormat;
    use env_logger::Builder;
    use log::LevelFilter;
    use std::fs::OpenOptions;
    use std::io::Write;

    #[macro_export]
    macro_rules! rakeInfo {
        ($($arg:tt)*) => {
            log::info!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! rakeWarn {
        ($($arg:tt)*) => {
            log::warn!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! rakeError {
        ($($arg:tt)*) => {
            log::error!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! rakeDebug {
        ($($arg:tt)*) => {
            log::debug!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! rakeTrace {
        ($($arg:tt)*) => {
            trace!($($arg)*);
        };
    }

    pub fn init(path: &str) {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Failed to open log file");

        let mut builder = Builder::new();

        builder
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {:<5} {}] {}",
                    chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                    record.level(),
                    record.target(),
                    record.args()
                )
            })
            .target(env_logger::Target::Pipe(Box::new(file)));

        if std::env::var("RUST_LOG").is_ok() {
            builder.parse_env("RUST_LOG");
        } else {
            builder.filter_level(LevelFilter::Info);
        }

        builder.init();

        rakeDebug!("Logging to file initialized");
    }
}
