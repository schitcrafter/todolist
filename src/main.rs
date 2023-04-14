use std::sync::Arc;

use actix_web::{get, Responder, HttpResponse, HttpServer, App, post, web, delete};
use dashmap::DashMap;

struct AppState {
    pub state_map: Arc<DashMap<u64, String>>,
}

#[get("/state")]
async fn get_all_state(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&*data.state_map)
}

#[get("/state/{id}")]
async fn get_state(path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    let state = data.state_map.get(&path.into_inner());
    let state = state.map(|prev| prev.value().to_string()).unwrap_or_default();
    HttpResponse::Ok().body(state)
}

#[post("/state/{id}")]
async fn set_state(req_body: String, path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    data.state_map.insert(id, req_body.clone());
    HttpResponse::Ok().body(req_body)
}

#[delete("/state/{id}")]
async fn delete_state(path: web::Path<u64>, data: web::Data<AppState>) -> impl Responder {
    data.state_map.remove(&*path);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(
        AppState { state_map: Arc::new(DashMap::new()) }
    );
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(get_state)
            .service(set_state)
            .service(delete_state)
            .service(get_all_state)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
