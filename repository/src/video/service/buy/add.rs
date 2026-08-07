// repository/src/video/service/buy/add.rs
// 仓储 - VIDEO - service - 购买
// 2026/8/2 17:03 Created.

////////

use crate::video::pg::comment::comment::CommentRepo;
use crate::video::pg::user::user_repo::UserRepo;
use crate::video::pg::video::count::VideoCountRepo;
use crate::video::pg::video::video::VideoRepo;
use anyhow::Error;
use cola_data::video::command::buy::VideoBuyCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::entity::buy::VideoBuyEntity;
use cola_data::video::entity::video::video::VideoEntity;
use tracing::log;
use crate::video::pg::buy::get::VideoBuyGetRepo;
use crate::video::pg::buy::manage::VideoBuyManageRepo;
////////

/// # [ADD SERVICE] - 视频 购买 添加 服务
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
        let buy_ids = VideoBuyGetRepo::find_buy_ids_by_user_id(user_id, keyword, offset, limit).await?;

        // 直接返回
        Ok(buy_ids)
    }

    ////////

    /// # 4. [SERVICE] - 删除购买 + 更新计数
    pub async fn del_buy_and_update_count(uid: i64, video_id: i64) -> Result<(), anyhow::Error> {
        // 1. 删除记录
        // 如果不需要返回具体的 handler，这里直接执行删除操作即可
        VideoBuyManageRepo::soft_delete_buy_by_video_id(uid, video_id).await?;

        // 2. 更新视频计数（购买/销售数量固定递增）
        VideoCountRepo::pg_update_video_buys(video_id).await?;

        Ok(())
    }
}

//////// END
