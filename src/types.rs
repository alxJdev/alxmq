use serde::{Deserialize, Serialize};

//Requests and Responses
#[derive(Serialize, Deserialize)]
pub struct ToMainQueRequest {
    pub que_id: u8,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToMainQueResponse {
    pub que_item_key: String,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromMainQueRequest {
    pub que_id: u8,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromMainQueResponse {
    pub que_items: Vec<QueObject>,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ToResponseQueRequest {
    pub que_id: u8,
    pub key: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToResponseQueResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromResponseQueRequest {
    pub que_id: u8,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveFromResponseQueResponse {
    pub message: String,
    pub success: bool,
}

//Other Types
#[derive(Serialize, Deserialize)]
pub struct QueObject {
    pub key: String,
    pub message: String,
}