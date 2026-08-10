// service/src/cola_video/buy/add.rs
// 👤 服务 - ▶ 可乐视频 - 购买记录 - 发布
// 2026/8/2 17:03 Created.

////////

use anyhow::Error;
use cola_data::cola_video::command::buy::VideoBuyCommand;
use cola_data::cola_video::entity::buy::VideoBuyEntity;
use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::cola_video::pg::buy::get::VideoBuyGetRepo;
use repository::cola_video::pg::user::profile::UserRepo;
use repository::cola_video::pg::video::count::VideoCountRepo;
use tracing::log;

////////

/// # [BUY ADD SERVICE] - 发布
/// * `desc`: `▶可乐视频 - 👤 视频购买发布服务`
pub struct VideoBuyAddService;

// 构造实现
impl VideoBuyAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存购买 + 更新计数
    pub async fn save_buy_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: VideoBuyCommand,
    ) -> Result<VideoBuyEntity, anyhow::Error> {
        let buy_entity = VideoBuyEntity::default();

        // 联动更新计数器：购买的视频数量 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            // 购买字段在第四位：publish, liked, total_favorited, buyed
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户购买计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        Ok(buy_entity)
    }

    ////////

    /// # 2. [SERVICE] - 根据用户ID查找购买记录IDs
    /// * `user_id`  用户 ID
    /// * `offset`
    /// * `limit`
    pub async fn find_buy_ids_by_user_id(
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        // 直接调用 Repo 获取 Vec<i64>，并使用 ? 传播错误
        let buy_ids = VideoBuyGetRepo::find_video_ids_by_user_id(user_id, offset, limit).await?;

        // 直接返回
        Ok(buy_ids)
    }
}

//////// END
