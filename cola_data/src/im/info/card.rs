// cola_data/src/im/info/card.rs  -- IM - info - 名片
// 2026-07-07

use crate::im::entity::card::ContactCardEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardInfo {
    pub id: i64,
    pub user_id: i64,
    pub contact_type: i16,
    pub content: String,
    pub first_name: String,
    pub last_name: String,
    pub visibility: i16,
    pub status: i16,
    pub add_time: i64,
    pub upd_time: i64,
}

impl CardInfo {
    pub fn from_entity(entity: ContactCardEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            contact_type: entity.contact_type,
            content: entity.content,
            first_name: entity.first_name,
            last_name: entity.last_name,
            visibility: entity.visibility,
            status: entity.status,
            add_time: entity.add_time,
            upd_time: entity.upd_time,
        }
    }
}