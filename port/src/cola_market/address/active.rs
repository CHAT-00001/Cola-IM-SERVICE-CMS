// cola_video/port/view/active.rs
// 视频 - 端口 - 浏览 - 发布
// 2026/8/4 22:11 Created.

////////

////////

/// # [VIEW ADD] 浏览发布
/// * `desc`: `浏览记录发布`
#[async_trait::async_trait]
pub trait AddressActivePort:  Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 设置默认地址
    /// * `desc`: `用户设置默认的收件地址`
    async fn set_default_address(
        &self,
        address_id: i64, // 地址 ID
    ) -> anyhow::Result<()>;
}

//////// END