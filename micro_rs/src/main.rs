use actix_web::{get, middleware, App, HttpServer};
use anyhow::Result;
use tracing::*;
use tracing_actix_web::TracingLogger;

mod error;
mod otel;

#[get("/")]
async fn no_params() -> &'static str {
    let user_name = "davidskyberg@gmail.com";
    let span = span!(Level::INFO, "my_span");
    let _guard = span.enter();

    event!(Level::INFO, user_name, "something happened inside my_span");
    "Hello world!\r\n"
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to load env file");

    // Set up the OpenTelemetry subscriber
    otel::init_subscriber()?;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .service(no_params)
        //.wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run();

    info!("Rust Micro service is running on http://120.0.0.1:8080");
    info!("For telemitry tracing, browse to http://localhost:3000");

    server.await?;
    Ok(())
}
