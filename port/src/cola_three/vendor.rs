// port/src/cola_three/port/vendor.rs
// ⏩️ 端口 - 可乐音乐 - 厂商端口
// 2026/6/18 14:10 Created.

////////

use cola_data::cola_three::command::vendor::UpsertVendorCommand;
use cola_data::cola_three::info::vendor::VendorInfo;

////////

/// # [PORT] - 厂商端口
#[async_trait::async_trait]
pub trait VendorPort: Send + Sync {
    /// 新增/更新
    async fn upsert(&self, cmd: UpsertVendorCommand) -> anyhow::Result<VendorInfo>;

    /// 列表（全部）
    async fn list(&self) -> anyhow::Result<Vec<VendorInfo>>;

    /// 按 code 查询
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<VendorInfo>>;
}

//////// END
