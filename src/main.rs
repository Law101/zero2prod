use sqlx::PgPool;
use zero2prod::{configurations, startup, telemetry};
use secrecy::ExposeSecret;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration =
        configurations::get_configurations().expect("failed to read configuration. ");
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    startup::run(listener, connection)?.await
}
