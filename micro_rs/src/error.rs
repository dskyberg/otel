use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[allow(dead_code)]
    #[error("OTEL Error")]
    Otel,
}
