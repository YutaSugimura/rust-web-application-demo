use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};

#[get("/")]
async fn index(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("app: {:?}", req);

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    session.set("counter", counter)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .default_service(
            web::resource("")
                .route(web::get().to(p404))
                .route(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed)
                ),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}