use actix_web::{web, App, HttpServer};
use log::error;
use routes::services::auth::login::login;
use std::io::{Error, ErrorKind};
use utils::database::connect_to_db;

mod routes;
mod utils;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    match connect_to_db().await {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            return Err(Error::new(
                ErrorKind::Other,
                "Failed to connect to the database",
            ));
        }
    }

    HttpServer::new(|| {
        App::new().service(web::scope("/api").service(web::scope("/auth").service(login)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
