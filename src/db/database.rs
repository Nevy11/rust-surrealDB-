use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::pizza::Pizza;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    client: Surreal<Client>,
    name_space: String,
    db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "nevy11",
                password: "Skyworth.95",
            })
            .await?;
        client.use_ns("surreal").use_db("pizzas").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("Surreal"),
            db_name: String::from("Pizzas"),
        })
    }
    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let result = self.client.select("pizza").await;
        match result {
            Ok(all_pizzas) => Some(all_pizzas),
            Err(_) => None,
        }
    }
    pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let created_pizza = self
            .client
            .create(("pizza", new_pizza.uuid.clone()))
            .content(new_pizza)
            .await;
        match created_pizza {
            Ok(created) => created,
            Err(_) => None,
        }
    }
}
