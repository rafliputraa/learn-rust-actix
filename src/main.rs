use actix_web::{get, post, patch, Responder, HttpResponse, HttpServer, App};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

#[post("/buypizza")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying a pizza!")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().service(get_pizzas))
        .bind("localhost:8080")?
        .run()
        .await
}
