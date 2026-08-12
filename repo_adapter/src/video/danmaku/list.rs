// repo_adapter/src/cola_video/danmaku/list.rs
// 🔌 插头 - VIDEO - 弹幕 - 弹幕列表
// 2026/8/6 18:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use port::cola_video::danmaku::list::VideoDanmakuListPort;

////////

/// # [LIST ADAPTER] - danmaku 列表
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
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn get_danmaku_by_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DanmakuInfo>, i64)> {
        todo!()
    }
}

//////// END
