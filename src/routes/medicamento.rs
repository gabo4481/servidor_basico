use actix_web::{web, HttpResponse};
use tera::Tera;
use sqlx::{FromRow, PgPool};
use serde::{Deserialize,Serialize};
use std::sync::Mutex;

#[derive(FromRow, Deserialize, Serialize)]
struct Medicamento {
    id: i32,
    nombre: String,
    principio_activo: String,
    presentacion: String,
    precio: f64
}

#[derive(FromRow, Deserialize, Serialize)]
struct NuevoMedicamento {
    nombre: String,
    principio_activo: String,
    presentacion: String,
    precio: f64
}

async fn mostrar_medicamentos(pool: web::Data<PgPool>, tmpl: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let medicamentos = sqlx::query_as!(
        Medicamento,
        "SELECT * FROM medicamentos"
    )
    .fetch_all(pool.get_ref())
    .await
    .expect("Error al cargar los medicamentos");

    let mut context = tera::Context::new();
    context.insert("medicamentos", &medicamentos);

    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("medicamentos/M-mostrar.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

async fn crear_medicamento(tmpl: web::Data<Mutex<Tera>>) -> actix_web::Result<HttpResponse> {
    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("medicamentos/M-crear.html", &tera::Context::new()).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}
async fn insertar_medicamento(form: web::Form<NuevoMedicamento>,pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "INSERT INTO medicamentos (nombre, principio_activo, presentacion, precio) VALUES ($1, $2, $3, $4)",
        form.nombre,
        form.principio_activo,
        form.presentacion,
        form.precio
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/medicamentos")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}

async fn editar_medicamento(path: web::Path<i32>, tmpl: web::Data<Mutex<Tera>>, pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let medicamento = sqlx::query_as!(
        Medicamento,
        "SELECT * FROM medicamentos WHERE id = $1",
        path.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    .expect("Error al cargar el medicamento");

    let mut context = tera::Context::new();
    context.insert("medicamento", &medicamento);

    let tera = tmpl.lock().unwrap();
    let rendered = tera.render("medicamentos/M-editar.html", &context).unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}

async fn actualizar_medicamento(form: web::Form<Medicamento>,pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "UPDATE medicamentos SET nombre = $1, principio_activo = $2, presentacion = $3, precio = $4 WHERE id = $5",
        form.nombre,
        form.principio_activo,
        form.presentacion,
        form.precio,
        form.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/medicamentos")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}

async fn eliminar_medicamento(path: web::Path<i32>, pool: web::Data<PgPool>) -> actix_web::Result<HttpResponse> {
    let result = sqlx::query!(
        "DELETE FROM medicamentos WHERE id = $1",
        path.into_inner()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/medicamentos")).finish()),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
    }
}

pub fn configurar_rutas(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/medicamentos").route(web::get().to(mostrar_medicamentos)));
    cfg.service(web::resource("/medicamentos/crear_medicamento").route(web::get().to(crear_medicamento)));
    cfg.service(web::resource("/medicamentos/insertar_medicamento").route(web::post().to(insertar_medicamento)));
    cfg.service(web::resource("/medicamentos/editar_medicamento/{id}").route(web::get().to(editar_medicamento)));
    cfg.service(web::resource("/medicamentos/actualizar_medicamento").route(web::post().to(actualizar_medicamento)));
    cfg.service(web::resource("/medicamentos/borrar_medicamento/{id}").route(web::get().to(eliminar_medicamento)));
}