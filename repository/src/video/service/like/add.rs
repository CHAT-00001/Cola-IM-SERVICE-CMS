// repository/src/video/service/active
// 仓储 - VIDEO - service - like - 点赞
// 2026/8/2 17:14 Created.

////////

use crate::gis::pg::poi_like::LikeRepo;
use crate::pg_pool;
use crate::video::pg::like::add::VideoLikeAddRepo;
use crate::video::pg::video::count::VideoCountRepo;
use anyhow::Result;

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
    ) -> Result<bool, sqlx::Error> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if status > 0 { 1 } else { -1 };

        // 2. 🛢️ 保存点赞记录
        VideoLikeAddRepo::pg_save_video_like(uid, video_id, status).await?;

        // 3. 🛢️ 更新视频计数
        VideoCountRepo::pg_update_video_likes(video_id, increment).await?;

        Ok(true)
    }

    ////////

    /// # 2. [SERVICE] - 保存不喜欢记录 + 更新视频计数
    /// `描述` --
    pub async fn save_unlike_with_update_video_count(
        uid: i64,      // 操作者
        video_id: i64, // 目标视频
        status: i16,   // 状态码
    ) -> Result<bool, sqlx::Error> {
        // 1. 🛡️ 安全转换：翻译底层加减增量步长
        let increment = if status > 0 { 1 } else { -1 };

        // 2. 🛢️  保存不喜欢记录
        VideoLikeAddRepo::pg_save_video_unlike(uid, video_id, status).await?;

        // 3. 🛢️ 更新视频计数
        VideoCountRepo::pg_update_video_unlikes(video_id, increment).await?;

        Ok(true)
    }
}

//////// END
