// repository/src/cola_im/pg/contact.rs  -- 仓储 - IM - PG - 联系人
// 2026-07-07

use crate::pg_pool;
use cola_data::cola_im::entity::contacts::contact::ImContactEntity as ContactEntity;

pub struct ImContactRepo;

impl ImContactRepo {
    pub async fn save_contact(uid: i64, card_id: i64, remark_name: Option<String>) -> Result<ContactEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        let entity = sqlx::query_as::<_, ContactEntity>(
            "INSERT INTO cola_im.contact (owner_id, card_id, remark_name, add_time, upd_time) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(uid).bind(card_id).bind(remark_name).bind(now).bind(now)
        .fetch_one(&pool).await?;
        Ok(entity)
    }

    pub async fn find_contacts_by_uid(uid: i64, offset: i64, limit: i64) -> Result<Vec<ContactEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, ContactEntity>(
            "SELECT * FROM cola_im.contact WHERE owner_id = $1 AND deleted = false ORDER BY upd_time DESC LIMIT $2 OFFSET $3"
        )
        .bind(uid).bind(limit).bind(offset).fetch_all(&pool).await
    }

    pub async fn soft_delete_contact(uid: i64, card_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE cola_im.contact SET deleted = true, del_time = $1, upd_time = $1 WHERE owner_id = $2 AND card_id = $3")
            .bind(now).bind(uid).bind(card_id).execute(&pool).await?;
        Ok(())
    }

    pub async fn update_star(uid: i64, card_id: i64, is_stared: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE cola_im.contact SET is_stared = $1, upd_time = $2 WHERE owner_id = $3 AND card_id = $4")
            .bind(is_stared).bind(now).bind(uid).bind(card_id).execute(&pool).await?;
        Ok(())
    }

    pub async fn update_favorites(uid: i64, card_id: i64, favorites: bool) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE cola_im.contact SET favorites = $1, upd_time = $2 WHERE owner_id = $3 AND card_id = $4")
            .bind(favorites).bind(now).bind(uid).bind(card_id).execute(&pool).await?;
        Ok(())
    }

    pub async fn update_blocked(uid: i64, card_id: i64, blocked: bool) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE cola_im.contact SET blocked = $1, upd_time = $2 WHERE owner_id = $3 AND card_id = $4")
            .bind(blocked).bind(now).bind(uid).bind(card_id).execute(&pool).await?;
        Ok(())
    }
}