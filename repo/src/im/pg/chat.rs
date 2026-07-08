use crate::pg_pool;
use cola_data::im::entity::card::ContactCardEntity;

pub struct ImChatRepo;

impl ImChatRepo {
    pub async fn save_chat(uid: i64, title: &str) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO cola_im.chat (user_id, title, add_time, upd_time) VALUES ($1,$2,$3,$4)")
            .bind(uid).bind(title).bind(now).bind(now).execute(&pool).await?;
        Ok(())
    }
}