use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_pizzas))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
/*
how to connect rust backend with angular front end
using postman
diesel with rust
cargo-watch
*/
