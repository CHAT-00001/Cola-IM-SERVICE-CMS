// repo_adapter/src/video/collect.rs -- 适配器 - VIDEO - 收藏
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::port::collect::CollectRepo;
use repository::video::service::collect::CollectService;

////////

pub struct CollectPortAdapter;

#[async_trait]
impl CollectRepo for CollectPortAdapter {

    /// # 1. [PORT] - 保存收藏记录 + 更新收藏数量
    async fn save_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()> {
        CollectService::save_collect_and_update_count(uid, video_id, cola_data::video::command::collect::CollectCommand::default()).await?;
        Ok(())
    }

    // # 2. [PORT] - 编辑收藏记录
    async fn edit_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()> {
        CollectService::save_collect_and_update_count(uid, video_id, cola_data::video::command::collect::CollectCommand::default()).await?;
        Ok(())
    }

    /// # 3. [PORT] - 删除收藏记录
    async fn del_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()> {
        CollectService::del_collect_and_update_count(uid, video_id).await
    }

    /// # 4.[PORT] - 根据用户ID获取收藏的视频ids
    async fn get_collect_ids_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        CollectService::find_collect_ids_by_user_id(user_id, None, offset, limit).await
    }
}

//////// END