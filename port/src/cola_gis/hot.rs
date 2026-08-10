// /hotlist  -- 上热门 服务端口
// 2026/7/7

////////

use cola_data::cola_gis::command::hotlist::HotlistCommand;

////////

/// # [SERVICE] - 上热门
#[async_trait::async_trait]
pub trait HotlistRepo: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存
    async fn save_hotlist_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: HotlistCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_hotlist_record(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()>;
}