mod api;
mod events;
mod utils;

use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use tokio::signal;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = match Database::connect(&database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error connecting to the database: {}", err);
            return;
        }
    };

    // Run database migrations
    if let Err(err) = Migrator::up(&connection, None).await {
        eprintln!("Error running migrations: {}", err);
        return;
    }

    tokio::task::spawn(async {
        events::monitor_events().await;
    });

    // Set up API routes
    let routes = api::routes::setup_routes(connection.clone()).with(warp::log("api"));

    let addr: SocketAddr = ([0, 0, 0, 0], 8000).into();
    println!("🚀 Server started successfully at {}", addr);

    // Start the server and await for shutdown signal
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(addr, shutdown_signal());

    // Await the server to finish running
    server.await;

    println!("🔴 Server shut down gracefully.");
}

// Function to handle shutdown signal
async fn shutdown_signal() {
    // Wait for Ctrl+C
    let _ = signal::ctrl_c().await;
    println!("🔴 Shutting down server...");
}
