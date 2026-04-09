use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use std::fs;

pub fn init_logging() -> (tracing_appender::non_blocking::WorkerGuard, tracing_appender::non_blocking::WorkerGuard) {
    let _ = fs::create_dir_all("logs");

    let console_layer = fmt::layer()
        .with_target(false);

    let all_file_appender = tracing_appender::rolling::never("logs", "all.log");
    let (all_file_writer, guard_all) = tracing_appender::non_blocking(all_file_appender);
    let all_file_layer = fmt::layer()
        .with_writer(all_file_writer)
        .with_ansi(false);

    let err_file_appender = tracing_appender::rolling::never("logs", "errors.log");
    let (err_file_writer, guard_err) = tracing_appender::non_blocking(err_file_appender);
    let err_file_layer = fmt::layer()
        .with_writer(err_file_writer)
        .with_ansi(false)
        .with_filter(LevelFilter::ERROR);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("error,miamtrix=trace")
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(all_file_layer)
        .with(err_file_layer)
        .init();

    (guard_all, guard_err)
}
