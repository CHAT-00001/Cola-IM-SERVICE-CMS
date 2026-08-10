// repo_adapter/src/cola_video/hotlist/list.rs
// 🔌 插头 - 可乐视频 - 上热门 - 列表
// 2026/8/6 19:04 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::hotlist::VideoHotlistInfo;
use port::cola_video::hotlist::list::VideoHotlistListPort;

////////

/// # [ADAPTER] - hotlist list
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistListAdapter;

#[async_trait]
impl VideoHotlistListPort for VideoHotlistListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户的
    async fn get_hotlist_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<VideoHotlistInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 视频的
    async fn get_hotlist_infos_by_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> Result<(Vec<VideoHotlistInfo>)> {
        todo!()
    }
}

//////// END
