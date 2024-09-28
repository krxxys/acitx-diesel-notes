use actix_web::{get, web::{self, route, scope}, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validator;
 
mod auth;

pub mod routes;
use routes::{auth_route, users_route::*};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = database::Database::new(); 
    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(validator); 
        App::new()
        .service(hello)
        .service(scope("/user")
            .route("/factory", web::post().to(routes::users_route::create_user))
            .route("/all_users", web::get().to(routes::users_route::get_all_users))
            .route("/info", web::post().to(routes::users_route::get_user_by_username)))
        .service(scope("/auth")
            .route("/login", web::post().to(routes::auth_route::login))
            .route("/auth_test", web::post().to(routes::auth_route::check_auth).wrap(auth))
        )
        .app_data(db_data.clone())
        
    })
    .bind(("127.0.0.1", 8080))?
    .run().await?;

    Ok(())

}
