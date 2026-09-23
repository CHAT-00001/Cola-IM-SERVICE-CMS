// repository/src/user/pg/profile/history.rs -- 用户资料历史 PostgreSQL 仓储
// 2026/9/23 Created.

////////

use crate::pg_pool;
use cola_data::app::page::PageInfo;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::entity::user_history::UserHistoryEntity;
use sqlx::{PgConnection, Row};

////////

/// # [REPOSITORY] - 用户资料与历史
/// * `desc`: `资料与历史记录在同一事务中更新`
pub struct UserHistoryRepo;

impl UserHistoryRepo {
    ////////

    /// # 1. [REPOSITORY] - 更新资料并保存历史
    pub async fn update_profile(uid: i64, cmd: UpdateUserCommand) -> anyhow::Result<()> {
        let pool = pg_pool();
        let mut tx = pool.begin().await?;
        let previous = sqlx::query(
            r#"SELECT avatar, user_nickname FROM "cola_user"."user" WHERE id = $1 FOR UPDATE"#,
        )
        .bind(uid)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        let old_avatar: Option<String> = previous.try_get("avatar")?;
        let old_nickname: Option<String> = previous.try_get("user_nickname")?;
        let result = sqlx::query(
            r#"UPDATE "cola_user"."user" SET
                user_nickname = COALESCE($2, user_nickname),
                signature = COALESCE($3, signature),
                avatar = COALESCE($4, avatar),
                avatar_thumb = COALESCE($5, avatar_thumb),
                bg_img = COALESCE($6, bg_img),
                sns_url = COALESCE($7, sns_url),
                email = COALESCE($8, email),
                phone = COALESCE($9, phone),
                birthday = COALESCE($10, birthday),
                lat = COALESCE($11, lat), lng = COALESCE($12, lng), updated_at = NOW()
                WHERE id = $1"#,
        )
        .bind(uid)
        .bind(&cmd.nickname)
        .bind(&cmd.signature)
        .bind(&cmd.avatar)
        .bind(&cmd.avatar_thumb)
        .bind(&cmd.bg_img)
        .bind(&cmd.sns_url)
        .bind(&cmd.email)
        .bind(&cmd.phone)
        .bind(cmd.birthday)
        .bind(&cmd.lat)
        .bind(&cmd.lng)
        .execute(&mut *tx)
        .await?;
        anyhow::ensure!(result.rows_affected() == 1, "用户不存在");

        for (kind, previous, updated) in [
            (1_i16, old_avatar, cmd.avatar),
            (2_i16, old_nickname, cmd.nickname),
        ] {
            if let (Some(previous), Some(updated)) = (previous, updated)
                && previous != updated
                && !previous.trim().is_empty()
            {
                Self::insert_history(&mut tx, uid, kind, &previous).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    ////////

    /// # 2. [REPOSITORY] - 分页查询用户历史
    pub async fn list(
        uid: i64,
        is_avatar: bool,
        page: i64,
        qty: i64,
    ) -> anyhow::Result<(Vec<UserHistoryEntity>, PageInfo)> {
        let pool = pg_pool();
        let kind = if is_avatar { 1_i16 } else { 2_i16 };
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM \"cola_user\".\"user_history\" WHERE uid = $1 AND history_type = $2 AND status = 1")
            .bind(uid).bind(kind).fetch_one(&pool).await?;
        let rows = sqlx::query_as::<_, UserHistoryEntity>(
            "SELECT id, uid, value, status, created_at FROM \"cola_user\".\"user_history\" WHERE uid = $1 AND history_type = $2 AND status = 1 ORDER BY created_at DESC, id DESC LIMIT $3 OFFSET $4",
        )
        .bind(uid).bind(kind).bind(qty).bind((page - 1) * qty).fetch_all(&pool).await?;
        Ok((
            rows,
            PageInfo {
                page,
                qty,
                has_more: page * qty < total,
            },
        ))
    }

    ////////

    /// # 3. [REPOSITORY] - 启用自己的有效历史
    pub async fn activate(uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        let mut tx = pool.begin().await?;
        let kind = if is_avatar { 1_i16 } else { 2_i16 };
        let row = sqlx::query("SELECT value FROM \"cola_user\".\"user_history\" WHERE id = $1 AND uid = $2 AND history_type = $3 AND status = 1 FOR UPDATE")
            .bind(id).bind(uid).bind(kind).fetch_optional(&mut *tx).await?;
        let value: String = row
            .ok_or_else(|| anyhow::anyhow!("历史记录不存在"))?
            .try_get("value")?;
        let column = if is_avatar { "avatar" } else { "user_nickname" };
        let current_query =
            format!("SELECT {column} FROM \"cola_user\".\"user\" WHERE id = $1 FOR UPDATE");
        let current_row = sqlx::query(&current_query)
            .bind(uid)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        let current_value: Option<String> = current_row.try_get(column)?;
        let query = format!(
            "UPDATE \"cola_user\".\"user\" SET {column} = $1, updated_at = NOW() WHERE id = $2"
        );
        let updated = sqlx::query(&query)
            .bind(&value)
            .bind(uid)
            .execute(&mut *tx)
            .await?;
        anyhow::ensure!(updated.rows_affected() == 1, "用户不存在");
        if let Some(current_value) =
            current_value.filter(|current| current != &value && !current.trim().is_empty())
        {
            Self::insert_history(&mut tx, uid, kind, &current_value).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 逻辑删除自己的历史
    pub async fn delete(uid: i64, is_avatar: bool, id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        let kind = if is_avatar { 1_i16 } else { 2_i16 };
        let result = sqlx::query("UPDATE \"cola_user\".\"user_history\" SET status = 0, updated_at = NOW() WHERE id = $1 AND uid = $2 AND history_type = $3 AND status = 1")
            .bind(id).bind(uid).bind(kind).execute(&pool).await?;
        anyhow::ensure!(result.rows_affected() == 1, "历史记录不存在");
        Ok(())
    }

    ////////

    async fn insert_history(
        connection: &mut PgConnection,
        uid: i64,
        kind: i16,
        value: &str,
    ) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO \"cola_user\".\"user_history\" (uid, history_type, value) VALUES ($1, $2, $3)")
            .bind(uid).bind(kind).bind(value).execute(&mut *connection).await?;
        Ok(())
    }
}

//////// END
