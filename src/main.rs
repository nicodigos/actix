use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "Hello, world!"
}

async fn home() -> impl Responder {
    "This is the home page"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/home", web::get().to(home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

