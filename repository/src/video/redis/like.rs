// repository/src/video/redis/like.rs -- VIDEO - REDIS - 评论点赞缓存仓储
// 2026/9/29 Created.

////////

use anyhow::Result;
use redis::AsyncCommands;

////////

/// # [REPOSITORY] - 视频评论点赞 Redis 仓储
/// * `desc`: `保存评论点赞关系和 action_id UUID v4 幂等结果`
pub struct CommentLikeRedisRepo;

impl CommentLikeRedisRepo {
    const RELATION_TTL: u64 = 30 * 24 * 60 * 60;
    const OPERATION_TTL: u64 = 7 * 24 * 60 * 60;

    ////////

    /// # 1. [REPOSITORY] - 评论点赞关系缓存 Key
    pub fn relation_key(uid: i64, comment_id: i64) -> String {
        format!("video:comment:like:{uid}:{comment_id}")
    }

    ////////

    /// # 2. [REPOSITORY] - action_id 幂等缓存 Key
    pub fn operation_key(uid: i64, action_id: &str) -> String {
        format!("video:comment:like:operation:{uid}:{action_id}")
    }

    ////////

    /// # 3. [REPOSITORY] - 写入评论点赞关系
    pub async fn set_relation(uid: i64, comment_id: i64, state: bool) -> Result<()> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let _: () = conn
            .set_ex(
                Self::relation_key(uid, comment_id),
                if state { "1" } else { "0" },
                Self::RELATION_TTL,
            )
            .await?;
        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 查询评论点赞关系
    pub async fn get_relation(uid: i64, comment_id: i64) -> Result<Option<bool>> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let value: Option<String> = conn.get(Self::relation_key(uid, comment_id)).await?;
        Ok(value.map(|state| state == "1"))
    }

    ////////

    /// # 5. [REPOSITORY] - 写入 action_id 幂等结果
    pub async fn set_operation_result(uid: i64, action_id: &str, state: bool) -> Result<()> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let _: () = conn
            .set_ex(
                Self::operation_key(uid, action_id),
                if state { "1" } else { "0" },
                Self::OPERATION_TTL,
            )
            .await?;
        Ok(())
    }

    ////////

    /// # 6. [REPOSITORY] - 查询 action_id 幂等结果
    pub async fn get_operation_result(uid: i64, action_id: &str) -> Result<Option<bool>> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let value: Option<String> = conn.get(Self::operation_key(uid, action_id)).await?;
        Ok(value.map(|state| state == "1"))
    }
}

////////

/// # [REPOSITORY] - 视频评论权限 Redis 仓储
/// * `desc`: `缓存视频是否存在及是否允许评论的检查结果`
pub struct VideoCommentAccessRedisRepo;

impl VideoCommentAccessRedisRepo {
    const TTL: u64 = 60;

    ////////

    /// # 1. [REPOSITORY] - 视频评论权限缓存 Key
    pub fn key(video_id: i64) -> String {
        format!("video:comment:access:{video_id}")
    }

    ////////

    /// # 2. [REPOSITORY] - 获取视频评论权限缓存
    pub async fn get(video_id: i64) -> Result<Option<String>> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let value: Option<String> = conn.get(Self::key(video_id)).await?;
        Ok(value)
    }

    ////////

    /// # 3. [REPOSITORY] - 设置视频评论权限缓存
    pub async fn set(video_id: i64, access: &str) -> Result<()> {
        let db = app_config::GLOBAL_DB
            .get()
            .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB 未初始化"))?;
        let mut conn = db.redis_conn.clone();
        let _: () = conn.set_ex(Self::key(video_id), access, Self::TTL).await?;
        Ok(())
    }
}

//////// END
