// repo/src/auth/pg/session.rs  -- 仓储 - 认证 - session（会话）
// 2026/5/23 07:15

////////

use cola_data::auth::entity::session::AuthSessionEntity;
use crate::pg_pool;

////////

/// # 统一的登录会话查询字段（完全对应你最新的 AuthSessionEntity 字段）
const SESSION_COLUMNS: &str = r#"
    id, user_id, access_token, refresh_token, client_id,
    device_id, access_expired_at,refresh_expired_at, last_active_at,
    status, created_time, updated_time
"#;


/// # [REPOSITORY] -
pub struct SessionRepo;

impl SessionRepo {

    ////////

    /// # 1. [REPOSITORY] - 插入新登录会话并“制裁”同平台旧设备
    /// 逻辑：在插入新 Session 前，将该用户在同平台下的其他活跃 Session 置为 -1 (被挤下线)
    pub async fn insert_session_with_kickout(
        entity: AuthSessionEntity, // 直接获取所有权
    ) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let mut tx = pool.begin().await?;

        // ① 制裁逻辑：更新同用户、同平台下的旧设备
        sqlx::query(
            r#"
        UPDATE "auth_session"
        SET status = -1, updated_time = NOW()
        WHERE user_id = $1 AND platform = $2 AND status = 1 AND device_id != $3
        "#
        )
            .bind(entity.user_id)
            .bind(entity.platform)
            .bind(&entity.device_id)
            .execute(&mut *tx)
            .await?;

        // ② 插入新记录
        // 注意：确保 VALUES 列表与 bind 顺序严格对应
        let id: i64 = sqlx::query_scalar(
            r#"
        INSERT INTO "auth_session" (
            user_id, access_token, refresh_token, client_id,
            device_id, access_expired_at, refresh_expired_at, last_active_at, status
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, )
        RETURNING id
        "#
        )
            .bind(entity.user_id)
            .bind(entity.access_token)        // 所有权转移
            .bind(entity.refresh_token)       // 所有权转移
            .bind(entity.client_id)
            .bind(entity.device_id)           // 所有权转移
            .bind(entity.access_expired_at)        // access_expired_at
            .bind(entity.refresh_expired_at)  // refresh_expired_at
            .bind(entity.last_active_at)
            .bind(entity.status)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(id)
    }

    ////////

    /// # 2. [REPOSITORY] - 注销登录 (主动退出)
    /// * session id 和 device id 双命中
    pub async fn update_session_by_device_id(session_id: &str, device_id: &str,) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let result = sqlx::query(
            r#"
            UPDATE "auth_session"
            SET status = 0, updated_time = NOW()
            WHERE id = $1 AND device_id = $2
            "#
        )
            .bind(session_id)
            .bind(device_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - 检查 Token 是否有效 (用于中间件鉴权)
    pub async fn find_active_session_by_token(
        refresh_token_hash: &str,
    ) -> Result<Option<AuthSessionEntity>, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp() as i32; // 转换为跟 Entity 一致的 i32 时间戳

        let sql = format!(
            "SELECT {} FROM \"auth_session\" WHERE refresh_token = $1 AND status = 1 AND expired_time > $2 LIMIT 1",
            SESSION_COLUMNS
        );

        sqlx::query_as::<_, AuthSessionEntity>(&sql)
            .bind(refresh_token_hash)
            .bind(now)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 获取用户当前所有在线设备列表 (用于安全中心展示)
    pub async fn find_online_devices_by_uid(
        user_id: i64,
    ) -> Result<Vec<AuthSessionEntity>, sqlx::Error> {
        let pool = pg_pool();

        let sql = format!(
            "SELECT {} FROM \"auth_session\" WHERE user_id = $1 AND status = 1 ORDER BY last_active_at DESC",
            SESSION_COLUMNS
        );

        sqlx::query_as::<_, AuthSessionEntity>(&sql)
            .bind(user_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 强制清理过期 Session (系统维护)
    pub async fn clean_expired_sessions() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp() as i32;

        let result = sqlx::query(r#"DELETE FROM "auth_session" WHERE expired_time < $1"#)
            .bind(now)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 5. [REPOSITORY] - 用户注销（单设备）
    /// * 机制：根据 user_id 和 device_id 唯一定位单端会话，将其状态标记为失效
    pub async fn logout_session(user_id: i64, device_id: &str) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();

        // 💡 优化点：
        // 1. 使用 status = 0 进行软删除，方便后续审计
        // 2. 双重过滤：user_id + device_id 确保只影响当前设备
        // 3. 将 access_expired_at 和 refresh_expired_at 置为当前时间，立即失效
        let result = sqlx::query(
            r#"UPDATE "auth_session"
           SET status = 0,
               access_expired_at = $1,
               refresh_expired_at = $1,
               updated_time = $2
           WHERE user_id = $3 AND device_id = $4 AND status = 1"#
        )
            .bind(now)
            .bind(chrono::Utc::now())
            .bind(user_id)
            .bind(device_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}


//////// END