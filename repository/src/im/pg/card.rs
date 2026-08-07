use crate::pg_pool;
use cola_data::im::entity::contacts::contact::ImContactEntity as ContactCardEntity;

pub struct ImCardRepo;

impl ImCardRepo {
    pub async fn save_card(uid: i64, first_name: &str, last_name: &str, content: &str) -> Result<ContactCardEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query_as::<_, ContactCardEntity>(
            "INSERT INTO cola_im.card (user_id, first_name, last_name, content, add_time, upd_time) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"
        ).bind(uid).bind(first_name).bind(last_name).bind(content).bind(now).bind(now).fetch_one(&pool).await
    }

    pub async fn find_card_by_id(card_id: i64) -> Result<Option<ContactCardEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, ContactCardEntity>("SELECT * FROM cola_im.card WHERE id = $1").bind(card_id).fetch_optional(&pool).await
    }

    pub async fn find_cards_by_uid(uid: i64, offset: i64, limit: i64) -> Result<Vec<ContactCardEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, ContactCardEntity>("SELECT * FROM cola_im.card WHERE user_id = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3")
            .bind(uid).bind(limit).bind(offset).fetch_all(&pool).await
    }

    pub async fn delete_card(uid: i64, card_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query("UPDATE cola_im.card SET status = -1 WHERE id = $1 AND user_id = $2").bind(card_id).bind(uid).execute(&pool).await?;
        Ok(())
    }
}