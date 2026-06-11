// service/like.rs  -- 服务层 - 点赞
// 2026/6/10 04:52

////////

use crate::pg_pool;
use crate::video::pg::count::CountRepo;
use crate::video::pg::like::LikeRepo;
use anyhow::Result;

////////

/// # [SERVICE] - 点赞
pub struct LikeService;

// 构造函数
impl LikeService {
    ////////

    /// # 1. [SERVICE] - 保存/取消点赞记录 + 更新视频计数
    pub async fn save_like_with_update_video_count(
        uid: i64,
        video_id: i64,
        is_like: bool,
    ) -> Result<bool, sqlx::Error> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if is_like { 1 } else { -1 };

        // 2. 🌟 顺滑调用：无论真假，直接把 is_like 塞给 Repo 的 UPSERT 逻辑，一行搞定流水！
        LikeRepo::pg_save_video_like(uid, video_id, is_like).await?;

        // 3. 更新视频计数表
        CountRepo::pg_update_video_likes(video_id, increment).await?;

        Ok(true)
    }

    ////////

    /// # 2. [SERVICE] - 保存不喜欢记录 + 更新视频计数
    /// `描述` --
    pub async fn save_unlike_with_update_video_count(
        uid: i64,
        video_id: i64,
        is_unlike: bool,
    ) -> Result<bool, sqlx::Error> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if is_unlike { 1 } else { -1 };

        // 2. 🌟 顺滑调用：无论真假，直接把 is_like 塞给 Repo 的 UPSERT 逻辑，一行搞定流水！
        LikeRepo::pg_save_video_unlike(uid, video_id, is_unlike).await?;

        // 3. 更新视频计数表
        CountRepo::pg_update_video_unlikes(video_id, increment).await?;

        Ok(true)
    }

    ////////

    /// # 2. [SERVICE] - 保存收藏记录 + 更新视频计数
    pub async fn save_collect_with_update_video_count(uid: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();

        // 1:1 匹配你在 UserEntity 中定义的物理字段 perm_id
        let sql = r#"
            UPDATE "video"
            SET collects = COALESCE(collects, 0) + $2
            WHERE id = $1
            LIMIT 1
        "#;

        let perm_id: (i16,) = sqlx::query_as(sql).bind(uid).fetch_one(&pool).await?;

        Ok(perm_id.0)
    }

    /// # 4. [SERVICE] - 获取用户点赞的ids
    pub async fn get_user_like_ids(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let ids = LikeRepo::find_like_record_by_user_id(user_id, limit, offset).await?;

        Ok(ids)
    }
    ////////
}

//////// END
