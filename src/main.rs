use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix Web")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8080;
    let addr = format!("{}:{}", host, port);

    println!("🚀 Server starting at http://{}", addr);

    HttpServer::new(|| App::new().service(index))
        .bind(&addr)?
        .run()
        .await
}
