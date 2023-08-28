use actix_cors::Cors;
use actix_web::{ web, App, HttpServer };

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allowed_methods(vec!["GET"]);

        App::new()
            .wrap(cors)
            .service(
                web::resource("/api/password/{length}").route(web::get().to(api::generate_password))
            )
    })
        .bind("0.0.0.0:8000")?
        .run().await
}
