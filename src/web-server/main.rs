use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn _manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/index.html")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[get("/count")]
async fn count(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource("/test2")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test2") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });

//     HttpServer::new(move || {
//         App::new()
//             .app_data(counter.clone())
//             .service(count)
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("My cool web server"),
//             }))
//             .service(
//                 web::scope("/app")
//                     .service(index)
//                     .service(hello)
//                     .service(echo)
//                     .route("/hey", web::get().to(manual_hello)),
//             )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
