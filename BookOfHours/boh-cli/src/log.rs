use tracing::{info, error, instrument};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub(crate) fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
}