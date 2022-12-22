use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prodd".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("should read configuration file");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("should connect to db");
    run(listener, connection_pool)?.await
}
