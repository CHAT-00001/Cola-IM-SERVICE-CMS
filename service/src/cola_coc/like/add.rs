// service/src/cola_video/like/add.rs
// 👤 服务 - ▶ 可乐视频 - 点赞 - 发布
// 2026/8/2 17:14 Created.

////////

use anyhow::Result;
use repository::video::pg::like::add::VideoLikeAddRepo;
use repository::video::pg::video::count::VideoCountRepo;

////////

/// # [SERVICE] - 点赞
pub struct VideoLikeAddService;

// 构造函数
impl VideoLikeAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存/取消点赞记录 + 更新视频计数
    pub async fn save_like_with_update_video_count(
        uid: i64,      // 操作者
        video_id: i64, // 目标视频
        status: i16,   // 状态码
    ) -> Result<bool> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if status > 0 { 1 } else { -1 };

        // 2. 🛢️ 保存点赞记录
        VideoLikeAddRepo::pg_save_video_like(uid, video_id, status)
            .await
            .map_err(|e| anyhow::anyhow!("保存视频点赞记录失败: {}", e))?;

        // 3. 🛢️ 更新视频计数
        VideoCountRepo::pg_update_video_likes(video_id, increment)
            .await
            .map_err(|e| anyhow::anyhow!("更新视频点赞计数失败: {}", e))?;

        Ok(true)
    }

    ////////

    /// # 2. [SERVICE] - 保存不喜欢记录 + 更新视频计数
    /// `描述` --
    pub async fn save_unlike_with_update_video_count(
        uid: i64,      // 操作者
        video_id: i64, // 目标视频
        status: i16,   // 状态码
    ) -> Result<bool> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if status > 0 { 1 } else { -1 };

        // 2. 🛢️  保存不喜欢记录
        VideoLikeAddRepo::pg_save_video_unlike(uid, video_id, status)
            .await
            .map_err(|e| anyhow::anyhow!("保存视频踩记录失败: {}", e))?;

        // 3. 🛢️ 更新视频计数
        VideoCountRepo::pg_update_video_unlikes(video_id, increment)
            .await
            .map_err(|e| anyhow::anyhow!("更新视频踩计数失败: {}", e))?;

        Ok(true)
    }
}

//////// END
