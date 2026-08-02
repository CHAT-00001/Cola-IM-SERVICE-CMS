// cola_data/src/video/port/hotlist.rs  -- 上热门 服务端口
// 数据 - VIDEO - port - hotlist 上热门
// 2026/6/10 08:31

////////

use crate::video::command::hotlist::HotlistCommand;

////////

/// # [SERVICE] - 上热门
#[async_trait::async_trait]
pub trait HotlistRepo: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_hotlist_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: HotlistCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_hotlist_record(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;
}

//////// END