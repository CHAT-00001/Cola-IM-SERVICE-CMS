// repository/src/cola_live/pg/user_repo.rs
// ✅ REPOSITORY - LIVE 直播域用户初始化
// 2026/8/20 Created.

////////

use crate::pg_pool;
use anyhow::{Context, Result, anyhow};

////////

/// # [REPOSITORY] - 直播域用户仓储
pub struct LiveUserRepo;

impl LiveUserRepo {
    /// # 1. [REPOSITORY] - 初始化直播域用户
    /// * `desc`: `等级初始化为1，经验值初始化为0，重复调用保持幂等`
    pub async fn init_live_user(user_id: i64) -> Result<()> {
        if user_id <= 0 {
            return Err(anyhow!("用户ID必须大于0"));
        }

        let pool = pg_pool();
        sqlx::query(
            r#"
            INSERT INTO cola_live."user"
                (user_id, level, experience, author_level, author_experience, status)
            VALUES ($1, 1, 0, 1, 0, 1)
            ON CONFLICT (user_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .execute(&pool)
        .await
        .context("初始化直播域用户资料失败")?;

        Ok(())
    }
}

//////// END
