// service/src/cola_video/hotlist/add.rs
// 仓储 - ▶ 可乐视频 - 推荐 - 发布
// 2026/8/2 18:57 Created.

////////

use cola_data::cola_video::command::recommend::VideoRecommendCommand;
use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::video::pg::video::home::VideoRepo;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `▶ 可乐视频 - 👤 视频推荐发布服务`
pub struct VideoRecommendAddService;

impl VideoRecommendAddService {
    //

    ////////

    /// # 7. [SERVICE] - 推荐
    /// * `desc`: `▶ 可乐视频 - 👤 视频浏览获取服务`
    pub async fn save_recommend_and_update_count(
        _uid: i64,
        _cmd: VideoRecommendCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 创作者通过特殊权益将视频送上推荐流记录

        Ok(())
    }

    ////////

    /// # 7001. [SERVICE] - 获取用户发布的视频列表
    /// * `desc`: `▶ 可乐视频 - 👤 视频浏览获取服务`
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
