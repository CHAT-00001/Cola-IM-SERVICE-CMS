// repository/src/user/pg/view/add.rs
// 仓储层 - 用户中心 - pg - 浏览 - 发布
// 2026/8/6 23:45 Created.

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::user::entity::user::UserEntity;
use sqlx::{self, PgPool};

//////

/// # [ADD REPOSITORY] - 发布
/// * `desc`: `仓储用户黑名单发布仓储`
pub struct UserBlackAddRepo;

// 构造函数
impl UserBlackAddRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - ✅️ 保存浏览记录
    /// * `desc`: `浏览记录保存仓储`
    pub async fn save_view_record(
        uid: i64,       // 当前用户
        user_id: i64,   // 目标用户
        remark: String, // 备注
        status: i16,    // 状态 (1: 拉黑, 0: 取消)
    ) -> Result<u64, sqlx::Error> {
        // 返回受影响的行数或 ()
        let pool = pg_pool();

        // 使用 ON CONFLICT 处理“重复拉黑”的情况
        let query = r#"
        INSERT INTO "cola_user.view" (uid, user_id,remark, status, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (uid, user_id)
        DO UPDATE SET status = EXCLUDED.status, updated_at = NOW()
    "#;

        let result = sqlx::query(query)
            .bind(uid)
            .bind(user_id)
            .bind(remark)
            .bind(status)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 🔄 更新
    /// * `desc`: `浏览记录刷新仓储`
    pub async fn update_view_record(
        uid: i64,     // 当前用户
        user_id: i64, // 目标用户
    ) -> Result<u64, sqlx::Error> {
        // 返回受影响的行数或 ()
        let pool = pg_pool();

        // 移除黑名单：通常是通过 uid 和 user_id 找到记录，将状态置为 0（取消/解除），并更新时间
        let query = r#"
            UPDATE "cola_user.view"
            SET status = 0, updated_at = NOW()
            WHERE uid = $1 AND user_id = $2
        "#;

        let result = sqlx::query(query)
            .bind(uid)
            .bind(user_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }
}

//////// END
