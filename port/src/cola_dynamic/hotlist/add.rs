// port/src/cola_dynamic/hotlist/add.rs -- 端口 - 动态 - 上热门 - 发布
// 2026/8/5 00:02 Created.

////////

use cola_data::cola_dynamic::command::hotlist::DynamicHotlistCommand;

////////

/// # [ADD PORTS] - 上热门
/// * `desc`: `DYNAMIC - Hotlist Add Port.`
#[async_trait::async_trait]
pub trait HotlistAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 创建热门
    async fn create_hotlist(
        &self,
        uid: i64,                   // 操作者 ID
        dynamic_id: i64,            // 动态 ID
        cmd: DynamicHotlistCommand, // 发布命令
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 更新热门
    async fn update_hotlist(
        &self,
        uid: i64,                   // 操作者 ID
        dynamic_id: i64,            // 动态 ID
        cmd: DynamicHotlistCommand, // 发布命令
    ) -> anyhow::Result<(bool)>;
}

//////// END
