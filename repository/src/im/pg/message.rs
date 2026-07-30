use crate::pg_pool;
use cola_data::im::entity::message::MessageEntity;

pub struct ImMessageRepo;

impl ImMessageRepo {
    pub async fn save_message(uid: i64, content: &str) -> Result<MessageEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query_as::<_, MessageEntity>(
            "INSERT INTO cola_im.message (user_id, content, send_time, sync_time) VALUES ($1,$2,$3,$4) RETURNING *"
        ).bind(uid).bind(content).bind(now).bind(now).fetch_one(&pool).await
    }

    pub async fn find_messages_by_uid(uid: i64, sync_time: i64, limit: i64) -> Result<Vec<MessageEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, MessageEntity>(
            "SELECT * FROM cola_im.message WHERE user_id = $1 AND sync_time > $2 ORDER BY sync_time DESC LIMIT $3"
        ).bind(uid).bind(sync_time).bind(limit).fetch_all(&pool).await
    }

    pub async fn soft_delete_message(uid: i64, msg_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("UPDATE cola_im.message SET status = 0 WHERE id = $1 AND user_id = $2").bind(msg_id).bind(uid).execute(&pool).await?;
        Ok(())
    }
}