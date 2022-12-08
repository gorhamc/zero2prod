use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("should read configuration file");
    let address = format!("127.0.0.1;{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("should connect to db");
    run(listener, connection_pool)?.await
}
