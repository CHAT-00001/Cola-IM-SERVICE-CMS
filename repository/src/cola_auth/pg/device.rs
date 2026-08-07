// repository/src/cola_auth/pg/device.rs  -- 存储 - AUTH  - PG - 设备
// 2026/5/26 07:55 by wx: cestbon10080

////////

use sqlx::{PgPool, Postgres, Transaction};
use app_config::app_state::AppState;
use cola_data::cola_auth::entity::device::AuthDeviceEntity;

////////

/// # 1. 统一的设备查询字段 (1:1 严格对齐我们最新的 AuthDeviceEntity 属性)
const DEVICE_COLUMNS: &str = r#"
    id, user_id, device_sn, platform, device_name, os_version, app_version,
    access_token, refresh_token, last_ip, is_online, status,
    expired_time, last_active_at, created_time, updated_time
"#;

pub struct DeviceRepo;

impl DeviceRepo {

    /// # [REPOSITORY] - 核心登录：绑定/更新设备并执行同平台单设备挤下线
    /// * 核心逻辑：利用 PostgreSQL 的 `ON CONFLICT` 做设备级 UPSERT，并利用事务确保挤人原子性
    pub async fn login_and_kickout_device(
        app_state: &AppState,
        entity: &AuthDeviceEntity,
    ) -> Result<i64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        // 使用强隔离事务，保证“同平台下线”和“新令牌落地”绝对原子化
        let mut tx = pool.begin().await?;

        // 1. 踢人制裁逻辑：将当前用户、同平台、状态为正常的【其他设备】标记为“被挤下线” (-1)
        sqlx::query(
            r#"
            UPDATE public.auth_device
            SET status = -1, is_online = 0
            WHERE user_id = $1 AND platform = $2 AND status = 1 AND device_sn != $3
            "#
        )
            .bind(entity.user_id)
            .bind(entity.platform)
            .bind(&entity.device_sn) // 👈 字符串加 &，稳如泰山
            .execute(&mut *tx)
            .await?;

        // 2. 插入或覆盖设备：采用高性能 UPSERT，同一个用户在同一个设备上再次登录时只更新令牌和指标
        let row: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO public.auth_device (
                user_id, device_sn, platform, device_name, os_version, app_version,
                access_token, refresh_token, last_ip, is_online, status, expired_time, last_active_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 1, 1, $10, $11)
            ON CONFLICT (user_id, device_sn)
            DO UPDATE SET
                platform = EXCLUDED.platform,
                device_name = EXCLUDED.device_name,
                os_version = EXCLUDED.os_version,
                app_version = EXCLUDED.app_version,
                access_token = EXCLUDED.access_token,
                refresh_token = EXCLUDED.refresh_token,
                last_ip = EXCLUDED.last_ip,
                is_online = 1,
                status = 1,
                expired_time = EXCLUDED.expired_time,
                last_active_at = EXCLUDED.last_active_at
            RETURNING id
            "#
        )
            .bind(entity.user_id)
            .bind(&entity.device_sn)
            .bind(entity.platform)
            .bind(&entity.device_name)
            .bind(&entity.os_version)
            .bind(&entity.app_version)
            .bind(&entity.access_token)
            .bind(&entity.refresh_token)
            .bind(&entity.last_ip)
            .bind(entity.expired_time)
            .bind(entity.last_active_at)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(row.0)
    }

    /// # [REPOSITORY] - 主动退出登录 (安全注销)
    pub async fn logout_device(
        app_state: &AppState,
        user_id: i64,
        device_sn: &str,
    ) -> Result<u64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        let result = sqlx::query(
            r#"
            UPDATE public.auth_device
            SET status = 0, is_online = 0
            WHERE user_id = $1 AND device_sn = $2 AND status = 1
            "#
        )
            .bind(user_id)
            .bind(device_sn)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// # [REPOSITORY] - 核心鉴权：根据 Access Token 验证设备合法性 (高频中间件核心)
    pub async fn find_active_device_by_token(
        app_state: &AppState,
        access_token: &str,
    ) -> Result<Option<AuthDeviceEntity>, sqlx::Error> {
        let pool = &app_state.db.pg_pool;
        let now_ts = chrono::Utc::now().timestamp(); // 获取当前 i64 秒级时间戳

        let sql = format!(
            "SELECT {} FROM public.auth_device WHERE access_token = $1 AND status = 1 AND expired_time > $2 LIMIT 1",
            DEVICE_COLUMNS
        );

        sqlx::query_as::<_, AuthDeviceEntity>(&sql)
            .bind(access_token)
            .bind(now_ts)
            .fetch_optional(pool)
            .await
    }

    /// # [REPOSITORY] - 心跳同步：更新设备最近活跃时间与 IP (长连接网关高频复用)
    pub async fn update_device_heartbeat(
        app_state: &AppState,
        user_id: i64,
        device_sn: &str,
        current_ip: &str,
    ) -> Result<u64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;
        let now_ts = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            r#"
            UPDATE public.auth_device
            SET last_active_at = $1, last_ip = $2, is_online = 1
            WHERE user_id = $3 AND device_sn = $4 AND status = 1
            "#
        )
            .bind(now_ts)
            .bind(current_ip)
            .bind(user_id)
            .bind(device_sn)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// # [REPOSITORY] - 安全中心：获取当前用户所有正在运行/在线的设备列表
    pub async fn find_online_devices_by_uid(
        app_state: &AppState,
        user_id: i64,
    ) -> Result<Vec<AuthDeviceEntity>, sqlx::Error> {
        let pool = &app_state.db.pg_pool;

        let sql = format!(
            "SELECT {} FROM public.auth_device WHERE user_id = $1 AND status = 1 ORDER BY last_active_at DESC",
            DEVICE_COLUMNS
        );

        sqlx::query_as::<_, AuthDeviceEntity>(&sql)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// # [REPOSITORY] - 后台断线僵尸维护：批量强制下线过期超时的设备
    pub async fn offline_expired_devices(app_state: &AppState) -> Result<u64, sqlx::Error> {
        let pool = &app_state.db.pg_pool;
        let now_ts = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            r#"
            UPDATE public.auth_device
            SET is_online = 0
            WHERE expired_time < $1 AND is_online = 1
            "#
        )
            .bind(now_ts)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}