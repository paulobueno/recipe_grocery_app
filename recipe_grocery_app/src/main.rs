use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod models;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_grocery_list(info: web::Json<models::RecipeUrl>) -> impl Responder {
    // Placeholder logic to process the recipe URL and generate a grocery list
    let grocery_list = format!("Grocery list for: {}", info.url);

    HttpResponse::Ok().body(grocery_list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/get_grocery_list", web::post().to(get_grocery_list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}