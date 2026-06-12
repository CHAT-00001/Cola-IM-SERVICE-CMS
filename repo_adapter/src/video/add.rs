// repo_adapter/src/video/add.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::port::add::AddPort;
use repo::video::service::add::AddService;

////////

/// # [ADD PORT] - 添加 端口 插头
pub struct AddPortAdapter;

////////

#[async_trait]
impl AddPort for AddPortAdapter {
    // * --------
    ////////

    /// # 1. [PORT] - 保存视频记录 + 更新用户视频数量
    async fn add_video(&self, uid: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddService::save_video_and_update_count(uid, data, 1).await?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 编辑视频
    async fn edit_video(&self, uid: i64, _video_id: i64, data: VideoCommand) -> anyhow::Result<()> {
        AddService::save_video_and_update_count(uid, data, 1).await?;
        Ok(())
    }

    ////////

    /// # 3. [PORT] - 删除单个视频
    async fn del_one_video(&self, _uid: i64, video_id: i64) -> anyhow::Result<()> {
        AddService::del_one_video_and_update_count(video_id).await?;
        Ok(())
    }

    ////////

    /// # 4. [PORT] - 遍历视频IDs批量删除视频
    async fn del_many_video(&self, _uid: i64, video_ids: Vec<i64>) -> anyhow::Result<()> {
        AddService::del_many_video_and_update_count(video_ids).await?;
        Ok(())
    }
}

//////// END
