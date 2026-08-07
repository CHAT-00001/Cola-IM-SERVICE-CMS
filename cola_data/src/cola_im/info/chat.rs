// cola_data/src/cola_im/info/chat.rs  -- IM - info - 聊天
// 2026-07-07

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatInfo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub chat_type: i16,
    pub avatar: Option<String>,
    pub status: i16,
    pub add_time: i64,
    pub upd_time: i64,
}