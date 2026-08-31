// servicey/src/cola_video/vide/get.rs
// 👤 服务 - 可乐视频 - 视频 - 获取
// 2026/8/2 12:40 Created.

////////

use anyhow::Result;
use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::video::pg::video::home::VideoRepo;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `▶ 可乐视频 - 👤 视频获取服务`
pub struct VideoGetService;

// 构造实现
impl VideoGetService {
    //

    ////////

    /// # 1. [SERVICE] - 查找最新的视频列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 2. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取热门视频列表失败: {}", e))
    }

    ////////

    /// # 3. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取推荐视频列表失败: {}", e))
    }

    ////////
}

//////// END
