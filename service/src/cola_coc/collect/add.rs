// service/src/cola_video/collect/add.rs
// 👤 服务 - ▶ 可乐视频  - 收藏 - 发布
// 2026/8/2 17:03 Created.

////////

use anyhow::Error;
use cola_data::cola_video::command::collect::CollectCommand;
use cola_data::cola_video::entity::collect::VideoCollectEntity;
use repository::cola_gis::pg::user::UserRepo;
use repository::video::pg::collect::get::CollectGetRepo;
use repository::video::pg::video::count::VideoCountRepo;
use tracing::log;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `▶ 可乐视频 - 👤 视频收藏发布服务`
pub struct VideoCollectAddService;

// 构造实现
impl VideoCollectAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存收藏 + 更新计数
    pub async fn save_collect_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: CollectCommand,
    ) -> Result<VideoCollectEntity, anyhow::Error> {
        let collect_entity = VideoCollectEntity::default();

        // 联动更新计数器：收藏的视频数量 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            // 收藏字段在第四位：publish, liked, total_favorited, collected
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                log::error!(
                    "[🤐 ADD SERVICE]: ❌️ 异步更新用户收藏计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        Ok(collect_entity)
    }
}

//////// END
