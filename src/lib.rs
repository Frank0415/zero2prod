use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = match req.match_info().get("name") {
        Some(n) => format!("Hello, {}!", n),
        None => format!("Hello, world!"),
    };
    name
}

async fn health_check(_req: HttpRequest) -> impl Responder{
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)    
}