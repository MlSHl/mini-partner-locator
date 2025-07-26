use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn greet() -> impl Responder {
    format!("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port: {port}");

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
