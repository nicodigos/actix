use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    name: String,
    any_number: i32,
}

#[get("/app-state")]
async fn display_app_state(data: web::Data<AppState>) -> impl Responder {
    let name = &data.name;
    let number = &data.any_number;
    HttpResponse::Ok().body(format!("<h1>AppState: {}, {}</h1>", name, number))
    
}

struct AnotherState {
    another_name: String
}

#[get("/another-state")]
async fn display_another_state(data: web::Data<AnotherState>) -> impl Responder {
    let name = &data.another_name;
    HttpResponse::Ok().body(format!("<h1>Another State: {}</h1>", name))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello World!</h1>")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hey there!</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(
            AppState{name: String::from("MyApp"), any_number: 10}
        )) 
        .app_data(web::Data::new(
            AnotherState{another_name:String::from("Example")}
        ))       
        .service(
            web::scope("/app")
                .service(hello)
                .service(echo)
                .route("hey", web::get().to(manual_hello)),
        )
        .service(
            web::scope("trying-state")
            .service(display_app_state)
            .service(display_another_state)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
