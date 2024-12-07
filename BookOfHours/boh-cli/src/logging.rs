use std::str::FromStr;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::{EnvFilter, Layer};

#[tracing::instrument]
pub(crate) fn get_env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or(
        EnvFilter::from_str("boh_cli=error").expect("Failed to create env filter from string"),
    )
}

pub(crate) fn init_tracing_subscriber(env_filter: EnvFilter) -> Result<(), TryInitError> {
    let subscriber = tracing_subscriber::Registry::default().with(
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_target(true)
            .with_thread_names(true)
            .with_line_number(true)
            .with_file(true)
            .with_filter(env_filter),
    );
    subscriber.try_init()
}
