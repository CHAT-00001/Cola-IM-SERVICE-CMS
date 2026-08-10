// port/src/cola_dynamic/danmaku/ids.rs
// ⏩️ 端口 - ⏹ 可乐动态 -  弹幕 - 获取ids
// 2026/8/9 05:07 Created.

////////

////////

/// # [IDS PORTS] - IDs
/// * `desc`: `⏹ 可乐动态 - 获取弹幕相关动态IDs端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuIDsPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取动态ID
    /// * `desc`: `可乐动态 - 根据弹幕ID获取动态ID
    async fn get_dynamic_id_by_danmaku_id(
        &self,
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(i64)>;
}
