use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use db::Database;
use models::{pizza::Pizza, BuyPizzaRequest, UpdatePizzaUrl};
use validator::Validate;

mod db;
mod models;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None => HttpResponse::Ok().body("Error!"),
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = db
                .add_pizza(Pizza::new(String::from(new_uuid), pizza_name))
                .await;
            match new_pizza {
                Some(created) => {
                    HttpResponse::Ok().body(format!("Created new pizza: {:?}", created))
                }
                None => HttpResponse::Ok().body(format!("Error buying a pizza!")),
            }
        }
        Err(_) => HttpResponse::Ok().body("pizza name is required"),
    }
    // HttpResponse::Ok().body("Buying a pizza!")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaUrl>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating a pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
/*
how to connect rust backend with angular front end
using postman
diesel with rust
cargo-watch
how to host your rust code for free
*/
