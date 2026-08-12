// repo_adapter/src/video/hotlist/add.rs
// 🔌 插头 - ▶ 可乐视频 - 上热门 - 发布
// 2026/8/6 19:03 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::hotlist::HotlistCommand;
use port::cola_video::hotlist::add::VideoHotlistAddPort;

////////

/// # [ADD ADAPTER] - hotlist add
/// * `desc`: `▶ 视频 - 上热门发布适配器`
#[derive(Debug, Default, Clone)]
pub struct hotlistaddPortAdapter;

// 构造实现
#[async_trait]
impl VideoHotlistAddPort for hotlistaddPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布
    async fn save_hotlist(
        &self,
        uid: i64,
        video_id: i64,
        cmd: HotlistCommand,
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 编辑
    async fn edit_hotlist(
        &self,
        uid: i64,
        hotlist_id: i64, // 上热门 ID
    ) -> Result<()> {
        todo!()
    }
}

//////// END
