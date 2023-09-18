use actix_web::{
    middleware,
    web::{self},
    App, HttpServer,
};

async fn handle_incoming_req() -> String {
    String::from("I can handle requests!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/").route(web::get().to(handle_incoming_req)), // .route(web::get().to(graphql_route)),
        )
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
// now go to http://127.0.0.1:8080/
