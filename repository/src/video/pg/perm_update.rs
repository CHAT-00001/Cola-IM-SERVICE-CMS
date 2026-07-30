// repository/src/video/service/pg/perm_update.rs  -- 仓储 - video - PG - 权限更新
// 2026/6/9 11:26

////////

use anyhow::{anyhow, Result};
use crate::pg_pool;

/// # [REPOSITORY] - 视频权限更新仓储
pub struct VideoPermUpdateRepo;


// 构造函数
impl VideoPermUpdateRepo {

    /// 辅助方法：执行通用更新
    async fn update_perm(video_id: i64, new_perm: i16, column: &str) -> Result<()> {
        let pool = pg_pool();
        // 构造动态列名的 SQL (注意：列名必须是硬编码的，防止 SQL 注入)
        let sql = format!(r#"UPDATE "video" SET {} = $1, updated_at = NOW() WHERE id = $2"#, column);

        let result = sqlx::query(&sql)
            .bind(new_perm)
            .bind(video_id)
            .execute(&pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow!("未找到视频 ID: {}，更新失败", video_id));
        }
        Ok(())
    }

    pub async fn update_video_visibility_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "visibility_perm").await
    }

    pub async fn update_video_comment_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "comment_perm").await
    }

    pub async fn update_video_danmaku_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "danmaku_perm").await
    }

    pub async fn update_video_collect_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "collect_perm").await
    }

    pub async fn update_video_download_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "download_perm").await
    }

    pub async fn update_video_buy_perm(video_id: i64, new_perm: i16) -> Result<()> {
        Self::update_perm(video_id, new_perm, "buy_perm").await
    }
}//////// END