use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MensagemModel {
    pub id: i32,
    pub nome: String,
    pub mensagem: Option<String>,
    #[serde(rename = "data")]
    pub data: chrono::NaiveDateTime,
}