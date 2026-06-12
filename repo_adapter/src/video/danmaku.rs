// repo_adapter/src/video/danmaku.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::port::danmaku::DanmakuRepo;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_data::video::info::danmaku::DanmakuInfo;
use repo::video::service::danmaku::DanmakuService;

////////
pub struct DanmakuPortAdapter;

#[async_trait]
impl DanmakuRepo for DanmakuPortAdapter {

    /// # 1. [PORT] - 保存弹幕 + 更新弹幕数量
    async fn save_danmaku_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: DanmakuCommand,
    ) -> anyhow::Result<()> {
        DanmakuService::save_danmaku_and_update_count(uid, video_id, cmd, 1).await?;
        Ok(())
    }

    /// # 2. [PORT] - 编辑弹幕(预设)
    async fn edit_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
        cmd: DanmakuCommand,
    ) -> anyhow::Result<()> {
        DanmakuService::delete_danmaku_and_update_count(uid, danmaku_id).await
    }

    /// # 3. [PORT] - 删除弹幕 + 更新弹幕数量
    async fn del_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
    ) -> anyhow::Result<()> {
        DanmakuService::delete_danmaku_and_update_count(uid, danmaku_id).await
    }

    /// # 4. [PORT] - 批量删除弹幕
    async fn del_danmakus_record(
        &self,
        uid: i64,
        danmaku_ids: Vec<i64>,
    ) -> anyhow::Result<()> {
        for id in danmaku_ids {
            DanmakuService::delete_danmaku_and_update_count(uid, id).await?;
        }
        Ok(())
    }

    /// # 5. [PORT] - 根据视频ID获取弹幕
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)> {
        let infos = DanmakuService::get_video_danmaku(video_id, play_time, qty, 0, 50).await?;
        let total = infos.len() as i64;
        Ok((infos, total))
    }

    /// # 6. [PORT] - 根据用户ID获取弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)> {
        let infos = DanmakuService::get_user_danmaku(uid, offset, limit).await?;
        let total = infos.len() as i64;
        Ok((infos, total))
    }
}

//////// END