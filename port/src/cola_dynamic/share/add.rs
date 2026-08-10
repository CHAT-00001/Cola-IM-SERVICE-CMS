// port/src/cola_dynamic/share/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 分享 - 发布
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;
use cola_data::cola_gis::command::share::ShareCommand;

////////

/// # [ADD PORTS] - 分享
/// `desc`: `⏹ 可乐动态 - 动态分享发布端口`
#[async_trait::async_trait]
pub trait DynamicShareAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送分享
    /// * `desc`: `🗣 USER` - `用户分享动态`
    async fn send_share(
        &self,
        uid: i64,          // UID
        dynamic_id: i64,   // 动态 ID
        cmd: ShareCommand, // 命令
    ) -> anyhow::Result<(DynamicCommentInfo)>;
}

//////// END
