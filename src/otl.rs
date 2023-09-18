use opentelemetry::sdk::metrics::controllers::BasicController;
use opentelemetry_otlp::WithExportConfig;

// https://github.com/open-telemetry/opentelemetry-rust/blob/d4b9befea04bcc7fc19319a6ebf5b5070131c486/examples/basic-otlp/src/main.rs#L35-L52
fn build_metrics_controller() -> BasicController {
    opentelemetry_otlp::new_pipeline()
        .metrics(
            opentelemetry::sdk::metrics::selectors::simple::histogram(Vec::new()),
            opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector(),
            opentelemetry::runtime::Tokio,
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .build()
        .expect("Failed to build metrics controller")
}

pub(crate) fn init_tracing(service: &'static str, version: &'static str) {
    // Configure otel exporter.
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_id_generator(opentelemetry::sdk::trace::RandomIdGenerator::default())
                .with_resource(opentelemetry::sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", service),
                    opentelemetry::KeyValue::new("service.version", version),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        // .install_simple()
        .expect("Not running in tokio runtime");

    // Compatible layer with tracing.
    let otel_trace_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let otel_metrics_layer = tracing_opentelemetry::MetricsLayer::new(build_metrics_controller());

    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::Layer::new())
        .with(otel_trace_layer)
        .with(otel_metrics_layer)
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}
