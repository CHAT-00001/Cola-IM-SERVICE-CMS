// servicey/src/cola_video/vide/feed.rs
// 👤 服务 - 可乐视频 - 视频 - feed
// 2026/6/10 17:44

////////

use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::video::pg::video::home::VideoRepo;

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
