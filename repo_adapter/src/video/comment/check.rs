// repo_adapter/src/video/comment/check.rs
// 🔌 适配器 - ▶ 视频 -  评论 - 检查
// 2026/8/9 20:48 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use redis::AsyncCommands;
use port::cola_video::comment::check::VideoCommentCheckPort;
use repository::video::pg::comment::check::VideoCommentCheckRepo;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `VIDEO - 视频评论检查适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoCommentCheckAdapter;

const VIDEO_COMMENT_DUP_CACHE_TTL: u64 = 7 * 24 * 60 * 60;

fn client_id_key(client_id: &str) -> String {
    format!("video:comment:dup:{}", client_id)
}

async fn get_dup_cache(client_id: &str) -> Result<Option<bool>> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let key = client_id_key(client_id);
    let value: Option<String> = conn.get(key).await?;
    Ok(value.map(|_| true))
}

async fn set_dup_cache(client_id: &str) -> Result<()> {
    let db = app_config::GLOBAL_DB
        .get()
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
    let mut conn = db.redis_conn.clone();
    let key = client_id_key(client_id);
    let _: () = conn.set_ex(key, "1", VIDEO_COMMENT_DUP_CACHE_TTL).await?;
    Ok(())
}

#[async_trait]
impl VideoCommentCheckPort for VideoCommentCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn check_health(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 状态
    async fn check_state(&self, uid: i64, comment_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 归属
    async fn is_owner(
        &self,
        uid: i64,
        user_id: i64,    // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 客户端幂等ID查重
    async fn exists_by_client_id(&self, client_id: String) -> Result<bool> {
        if let Ok(Some(_)) = get_dup_cache(&client_id).await {
            tracing::info!("[🔌 ADAPTER] - ✅️ 评论幂等缓存命中: client_id={}", client_id);
            return Ok(true);
        }

        let exists = VideoCommentCheckRepo::exists_by_client_id(&client_id).await?;
        if exists {
            if let Err(error) = set_dup_cache(&client_id).await {
                tracing::warn!("[🤐 ADAPTER] - ❌️ 评论幂等缓存回填失败: client_id={}, error={}", client_id, error);
            }
        }

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 评论幂等查重完成: client_id={}, exists={}",
            client_id,
            exists
        );
        Ok(exists)
    }
}

//////// END
