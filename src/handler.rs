use crate::{
    model::MensagemModel,
    schema::{CreateMensagemSchema, FilterOptions},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};
// use chrono::prelude::*;
use serde_json::json;

// http://localhost:8000/api/healthchecker to call this handler test
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Server is running ✅";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/mensagem")]
pub async fn mensagem_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        MensagemModel,
        "SELECT * FROM crud.mensagem ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all message items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let mensagem = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": mensagem.len(),
        "notes": mensagem
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/mensagem/")]
async fn create_mensagem_handler(
    body: web::Json<CreateMensagemSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        MensagemModel,
        "INSERT INTO crud.mensagem (nome,mensagem) VALUES ($1, $2) RETURNING *",
        body.nome.to_string(),
        body.mensagem.to_string() // Fix: Removed extra closing parenthesis
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(mensagem) => {
            let mensagem_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "mensagem": mensagem
            })});

            HttpResponse::Ok().json(mensagem_response)
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint") {
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
            } else {
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
            }
        }
    }
}

#[get("/mensagem/{id}")]
async fn get_mensagem_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mensagem_id = path.into_inner();
    let query_result = sqlx::query_as!(
        MensagemModel,
        "SELECT * FROM crud.mensagem WHERE id = $1",
        mensagem_id.to_string().parse::<i32>().unwrap()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(mensagem) => {
            let mensagem_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "mensagem": mensagem
            })});

            return HttpResponse::Ok().json(mensagem_response);
        }
        Err(_) => {
            let message = format!("Mensagem with ID: {} not found", mensagem_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(mensagem_list_handler)
        .service(create_mensagem_handler)
        .service(get_mensagem_handler);

    conf.service(scope);
}
