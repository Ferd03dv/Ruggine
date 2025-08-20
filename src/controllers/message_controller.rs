use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::services::message_service::MessageService;

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub sender_id: i64,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub m_id: i64,
    pub content: String,
    pub sent_at: String,
    pub sender_id: i64,
    pub group_id: i64,
}

impl From<crate::models::Message> for MessageResponse {
    fn from(message: crate::models::Message) -> Self {
        Self {
            m_id: message.m_id,
            content: message.content,
            sent_at: message.sent_at.to_string(),
            sender_id: message.sender_id,
            group_id: message.group_id,
        }
    }
}

pub async fn send_message(
    State(pool): State<SqlitePool>,
    Path(group_id): Path<i64>,
    Json(request): Json<CreateMessageRequest>,
) -> Result<Json<MessageResponse>, StatusCode> {
    let message_service = MessageService::new(pool);
    
    match message_service.send_message(group_id, request.sender_id, &request.content).await {
        Ok(message) => Ok(Json(MessageResponse::from(message))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_messages(
    State(pool): State<SqlitePool>,
    Path(group_id): Path<i64>,
) -> Result<Json<Vec<MessageResponse>>, StatusCode> {
    let message_service = MessageService::new(pool);
    
    match message_service.get_messages_by_group(group_id).await {
        Ok(messages) => {
            let responses: Vec<MessageResponse> = messages.into_iter().map(MessageResponse::from).collect();
            Ok(Json(responses))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
