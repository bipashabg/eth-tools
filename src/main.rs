use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod handlers;
mod utils;
mod routes;
mod txn_decoder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started server at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())  
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

