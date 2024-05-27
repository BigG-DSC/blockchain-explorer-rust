use crate::models::blockchain_status::BlockchainStatus;

use actix_web::{
    get,
    error::ResponseError,
    web::Json,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use dotenv;
use reqwest;
use tokio;
use derive_more::{Display};

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
        }
    }
}

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("NOW_NODES_KEY").expect("Could not find the API Key"))
        .send()
        .await.expect("Failed to get response")
        .text()
        .await.expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

#[get("/blockchain-status")]
pub async fn get_status() -> Result<Json<BlockchainStatus>, TaskError> {

    let task = Some(
        blockchain_status_request()
    );

    match task {
        Some(task) => Ok(Json(task)),
        None => Err(TaskError::TaskNotFound)
    }
}