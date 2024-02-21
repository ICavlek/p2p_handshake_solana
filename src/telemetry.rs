use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Function that creates a Registry that collects logging data.
/// It has four levels of information filter: info, error, debug and trace.
/// To have it more user friendly, json storage layer and bunyan formatting
/// layer have been added.
///
/// #Example
///
/// ```
/// use p2p_handshake_solana::telemetry::get_subscriber;
/// let subscriber = get_subscriber(
///     "p2p_handshake_solana".into(),
///     "info".into(),
///     std::io::stdout,
/// );
/// ```
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Function that initializes created subscriber.
///
/// #Example
///
/// ```
/// use p2p_handshake_solana::telemetry::{get_subscriber, init_subscriber};
/// let subscriber = get_subscriber(
///     "p2p_handshake_solana".into(),
///     "info".into(),
///     std::io::stdout,
/// );
/// init_subscriber(subscriber);
/// ```
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
