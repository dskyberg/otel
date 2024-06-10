use anyhow::Result;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    resource::{
        OsResourceDetector, ProcessResourceDetector, ResourceDetector, SdkProvidedResourceDetector,
        TelemetryResourceDetector,
    },
    trace as sdktrace, Resource,
};
use opentelemetry_semantic_conventions::resource as otel_resource;
use std::time::Duration;
use tracing_subscriber::{layer::*, util::*, EnvFilter};

fn init_resource() -> Result<Resource> {
    let os_resource = OsResourceDetector.detect(Duration::from_secs(0));
    let process_resource = ProcessResourceDetector.detect(Duration::from_secs(0));
    let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));
    let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));

    let provided = Resource::new(vec![
        KeyValue::new(otel_resource::SERVICE_NAME, "micro_rs"),
        KeyValue::new(otel_resource::SERVICE_NAMESPACE, "micro_rs"),
        KeyValue::new(otel_resource::SERVICE_VERSION, "0.1.0"),
        KeyValue::new(otel_resource::SERVICE_INSTANCE_ID, "127.0.0.1"),
        KeyValue::new(otel_resource::DEPLOYMENT_ENVIRONMENT, "development"),
    ]);

    Ok(sdk_resource
        .merge(&provided)
        .merge(&telemetry_resource)
        .merge(&os_resource)
        .merge(&process_resource))
}

fn init_tracer(endpoint: &str, resource: Resource) -> Result<sdktrace::Tracer> {
    Ok(opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint),
        )
        .with_trace_config(sdktrace::config().with_resource(resource))
        .with_batch_config(sdktrace::BatchConfigBuilder::default().build())
        .install_batch(opentelemetry_sdk::runtime::Tokio)?)
    //.expect("Unable to initialize OtlpPipeline for traces")
}

pub fn init_subscriber() -> Result<()> {
    let otlp_endpoint = std::env::var("OLTP_URL").unwrap_or("http://localhost:4317".to_string());
    let resource = init_resource()?;
    let tracer = init_tracer(&otlp_endpoint, resource)?;

    // Create a layer for sending traces to OTLP Receiver
    let traces_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(EnvFilter::from_default_env());

    // Create a layer for sending to console
    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .compact()
        .with_filter(EnvFilter::from_default_env());

    // Add the layers to the registry and start logs + traces
    tracing_subscriber::registry()
        .with(traces_layer)
        .with(stdout_layer)
        .try_init()?;
    //.expect("Could not init tracing registry");

    Ok(())
}
