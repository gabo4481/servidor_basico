//CRUD BASICO EN RUST

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;
use tera::Tera;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;

mod routes;
use routes::{farmacia, medicamento};

async fn inicio(tlmp: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let tera = tlmp.lock().unwrap();
    let rendered = tera.render("inicio.html", &tera::Context::new()).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}

async fn version(tlmp: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let tera = tlmp.lock().unwrap();
    let rendered = tera.render("version.html", &tera::Context::new()).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error al cargar la ruta de la Base de Datos");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let tera = Tera::new("src/templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Mutex::new(tera.clone())))
            .route("/", web::get().to(inicio))
            .route("/version", web::get().to(version))
            .configure(farmacia::configurar_rutas)
            .configure(medicamento::configurar_rutas)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    
}
