
use std::sync::{atomic::AtomicU32, Arc};

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, rt::time::Instant, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::routes::health_router::health_checker_handler;

mod models;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    visit_count: Arc<AtomicU32>,
    up_running: Instant,
    db: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("Hello, world!");

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {std::env::set_var("RUST_LOG", "actix_web=info");}
    }

    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
    let pgpool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await {
                Ok(pool) => {
                    println!("✅Connection to the DB is successfull!");
                    pool
                },
                Err(err) => {
                    println!("🔥 Failed to connect to the DB \n{:?}",err);
                    std::process::exit(1);
                },
            };

    let app_state = AppState {
                visit_count: Arc::new(AtomicU32::new(0)),
                up_running: Instant::now(),
                db: pgpool.clone()
            };
    println!("Server started successfully 🚀!");

    HttpServer::new(move ||{
        let cors = Cors::default()
                .allowed_origin("http://localhost:5173")
                .allowed_methods(vec!["GET","POST","PUT","DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .supports_credentials();
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .service(health_checker_handler)
            // .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
    
}


/*
    Setup DB
    - setup db pool

    Declare routes
    - declare cors middleware

    Start listening for requests on port x
*/