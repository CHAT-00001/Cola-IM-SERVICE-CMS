// repository/src/video/servic/feed.rs
// 仓储 - VIDEO - service - feed - 流
// 2026/6/10 17:44

////////

use crate::video::pg::video::video::VideoRepo;
use cola_data::video::entity::video::video::VideoEntity;

////////

/// # [FEED SERVICE] - 流服务
pub struct FeedService;

impl FeedService {
    //

    ////////

    /// # 1. [SERVICE] - 获取用户发布的视频列表
    pub async fn find_user_publish_list(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!("SERVICE: 获取用户{}发布的最新视频列表失败: {}", user_id, e)
            })
    }
}

//////// END
