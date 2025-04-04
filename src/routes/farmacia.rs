use actix_web::{web, HttpResponse};
use tera::Tera;
use sqlx::{FromRow, PgPool};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;

#[derive(FromRow, Deserialize, Serialize)]
struct Farmacia {
    id: i32,  
    nombre: String,
    direccion: String,  
    telefono: Option<String>    
}

#[derive(FromRow, Deserialize, Serialize)]
struct NuevaFarmacia {
    nombre: String,
    direccion: String,
    telefono: Option<String>,
}


async fn mostrar_farmacias(pool: web::Data<PgPool>, tmpl: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let farmacias = sqlx::query_as!(
        Farmacia,
        "SELECT * FROM farmacias"
    )
    .fetch_all(pool.get_ref())
    .await
    .expect("Error al cargar los farmacias");

    let mut context = tera::Context::new();
    context.insert("farmacias", &farmacias);

    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("farmacias/F-mostrar.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

async fn crear_farmacia(tmpl: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("farmacias/F-crear.html", &tera::Context::new()).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}
async fn insertar_farmacia(form: web::Form<NuevaFarmacia>,pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "INSERT INTO farmacias (nombre, direccion, telefono) VALUES ($1, $2, $3)",
        form.nombre,
        form.direccion,
        form.telefono
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/farmacias")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}

async fn editar_farmacia(path: web::Path<i32>, tmpl: web::Data<Mutex<Tera>>, pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let farmacia = sqlx::query_as!(
        Farmacia,
        "SELECT * FROM farmacias WHERE id = $1",
        path.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    .expect("Error al cargar el farmacia");

    let mut context = tera::Context::new();
    context.insert("farmacia", &farmacia);

    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("farmacias/F-editar.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

async fn actualizar_farmacia(form: web::Form<Farmacia>,pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "UPDATE farmacias SET nombre = $1, direccion = $2, telefono = $3 WHERE id = $4",
        form.nombre,
        form.direccion,
        form.telefono,
        form.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/farmacias")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}

async fn eliminar_farmacia(path: web::Path<i32>, pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "DELETE FROM farmacias WHERE id = $1",
        path.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/farmacias")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}



pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/farmacias").route(web::get().to(mostrar_farmacias)));
    cfg.service(web::resource("/farmacias/crear_farmacia").route(web::get().to(crear_farmacia)));
    cfg.service(web::resource("/farmacias/insertar_farmacia").route(web::post().to(insertar_farmacia)));
    cfg.service(web::resource("/farmacias/editar_farmacia/{id}").route(web::get().to(editar_farmacia)));
    cfg.service(web::resource("/farmacias/actualizar_farmacia").route(web::post().to(actualizar_farmacia)));
    cfg.service(web::resource("/farmacias/borrar_farmacia/{id}").route(web::get().to(eliminar_farmacia)));
}