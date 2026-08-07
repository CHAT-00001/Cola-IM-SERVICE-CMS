// repository/src/cola_im/pg/chat.rs
// 仓储 - IM - pg - chat 聊天会话
// 2026/7/7 14:00 Created.

////////

use crate::pg_pool;
use cola_data::cola_im::entity::profile::card::ContactCardEntity;

////////

/// # [REPOSITORY] - IM 聊天 仓储
pub struct ImChatRepo;

// 构造实现
impl ImChatRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 💾 保存聊天
    pub async fn save_chat(uid: i64, title: &str) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO cola_im.chat (user_id, title, add_time, upd_time) VALUES ($1,$2,$3,$4)")
            .bind(uid).bind(title).bind(now).bind(now).execute(&pool).await?;
        Ok(())
    }
}

//////// END