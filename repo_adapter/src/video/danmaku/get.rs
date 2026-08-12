// repo_adapter/src/cola_video/danmaku/get.rs
// 🔌 插头 - 可乐视频 - 弹幕 - 获取IDs
// 2026/8/6 18:56 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use port::cola_video::danmaku::get::VideoDanmakuGetPort;

////////

/// # [GET ADAPTER] - danmaku get
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuGetAdapter;

#[async_trait]
impl VideoDanmakuGetPort for VideoDanmakuGetAdapter {
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> Result<(Vec<DanmakuInfo>, i64)> {
        todo!()
    }

    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<DanmakuInfo>, i64)> {
        todo!()
    }
}

//////// END
