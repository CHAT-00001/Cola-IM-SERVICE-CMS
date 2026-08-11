// port/src/market/address/manage.rs
// ⏩️ 端口 - MARKET - 地址簿 - 管理端口
// 2026/8/4 22:11 Created.

////////

use cola_data::cola_video::info::view::VideoViewInfo;

////////

/// # [MANAGE PORTS]
/// * `desc`: `MARKET - 地址簿管理端口`
#[async_trait::async_trait]
pub trait AddressManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `查看所有的地址簿记录`
    /// * `condition`: `⚠️ 仅限管理员 / 运营人员`
    async fn admin_list(
        &self,
        uid: i64,                // 操作者 ID
        user_id: Option<i64>,    // 用户 ID
        video_id: Option<i64>,   // 视频 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<VideoViewInfo>), u64>;
}

//////// END
