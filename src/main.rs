use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().content_type("text/html").body("<h1>Hello World</h1>")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}