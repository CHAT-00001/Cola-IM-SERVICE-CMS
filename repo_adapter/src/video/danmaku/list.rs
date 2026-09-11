// repo_adapter/src/cola_video/danmaku/list.rs
// 🔌 适配器 - VIDEO - 弹幕 - 弹幕列表
// 2026/8/6 18:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use port::cola_video::danmaku::list::VideoDanmakuListPort;
use service::cola_video::danmaku::add::VideoDanmakuAddService;

////////

/// # [LIST ADAPTER] - danmaku 列表
/// * `desc`: `VIDEO - 视频弹幕列表适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuListAdapter;

#[async_trait]
impl VideoDanmakuListPort for VideoDanmakuListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 视频的
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> Result<(Vec<DanmakuInfo>, i64)> {
        let infos = VideoDanmakuAddService::get_video_danmaku(
            video_id,
            play_time,
            5,
            qty as i64,
            0,
        )
        .await?;
        let total = infos.len() as i64;
        Ok((infos, total))
    }

    ////////

    /// # 2. [ADAPTER] - 用户的
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DanmakuInfo>, i64)> {
        todo!()
    }
}

//////// END
