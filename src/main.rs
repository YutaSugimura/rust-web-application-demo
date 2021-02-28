use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    get, guard, middleware, web, App, HttpRequest, HttpResponse,
    HttpServer, Result,
};

#[get("/")]
async fn index(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("app: {:?}", req);

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[get("/hello")]
async fn hello(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/hello.html"))
    )
}

use rust_web::rand_func::rand_num;

#[get("/rand_page")]
async fn rand_page(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    rand_num();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/hello.html"))
    )
}


async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
            .service(hello)
            .service(rand_page)
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