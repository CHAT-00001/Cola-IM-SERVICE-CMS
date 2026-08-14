// repository/src/user/pg/vip/add.rs
// 仓储 - USER - vip - add 添加
// 2026/8/3 12:53 Created.
// 2026/8/6 实现：VIP 充值记录写入

////////

use crate::pg_pool;
use cola_data::cola_user::command::vip::VipCommand;
use sqlx;

////////

/// # [VIP REPO] - 贵宾充值仓储
pub struct VipAddRepo;

impl VipAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存 VIP 充值记录
    /// * `desc`: 在 user_vip 表中插入一条充值记录，同时更新 user 表的 is_vip 状态
    pub async fn pg_save_vip_record(
        uid: i64,       // 操作者
        target_id: i64, // 开通目标用户
        cmd: &VipCommand,
    ) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let now = chrono::Utc::now().timestamp();

        let id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO "user_vip" (uid, target_id, vip_type, pay_method, amount, remark, source, status, add_time, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 1, $8, NOW())
            RETURNING id
            "#,
        )
        .bind(uid)
        .bind(target_id)
        .bind(cmd.vip_type)
        .bind(cmd.pay_method)
        .bind(cmd.amount)
        .bind(&cmd.remark)
        .bind(&cmd.source)
        .bind(now)
        .fetch_one(&pool)
        .await?;

        // 同时更新 user 表的 is_vip 状态
        let _ = sqlx::query(
            r#"UPDATE "user" SET is_vip = 1, updated_at = NOW() WHERE id = $1"#,
        )
        .bind(target_id)
        .execute(&pool)
        .await?;

        Ok(id)
    }

    ////////

    /// # 2. [REPOSITORY] - 取消 VIP
    pub async fn pg_cancel_vip_record(
        uid: i64,
        target_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query(
            r#"UPDATE "user_vip" SET status = 0 WHERE uid = $1 AND target_id = $2 AND status = 1"#,
        )
        .bind(uid)
        .bind(target_id)
        .execute(&pool)
        .await?;

        // 同时取消 user 表的 is_vip
        let _ = sqlx::query(
            r#"UPDATE "user" SET is_vip = 0, updated_at = NOW() WHERE id = $1"#,
        )
        .bind(target_id)
        .execute(&pool)
        .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - 检查用户是否已开通 VIP
    pub async fn pg_check_vip_status(user_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let exists: Option<i64> = sqlx::query_scalar(
            r#"SELECT 1 FROM "user" WHERE id = $1 AND is_vip = 1 LIMIT 1"#,
        )
        .bind(user_id)
        .fetch_optional(&pool)
        .await?;

        Ok(exists.is_some())
    }
}

//////// END