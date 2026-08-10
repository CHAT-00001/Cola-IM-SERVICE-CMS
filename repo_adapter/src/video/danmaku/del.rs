// repo_adapter/src/video/danmaku/del.rs
// 🔌 插头 - ▶ 可乐视频 - 弹幕 - 删除
// 2026/8/6 18:56 Created.

////////

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::danmaku::del::VideoDanmakuDelPort;

////////

/// # [DELETE ADAPTER] - danmaku del
/// * `desc`: `▶ 视频 - 弹幕记录软删除适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuDelAdapter;

// 构造实现
#[async_trait]
impl VideoDanmakuDelPort for VideoDanmakuDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    async fn single_soft_del_record(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕 ID
    ) -> Result<(u64)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量
    async fn batch_soft_del_record(
        &self,
        uid: i64,
        danmaku_ids: Vec<i64>, // 弹幕 IDs
    ) -> Result<(u64)> {
        todo!()
    }
}

//////// END
