// repo_adapter/src/three/three_vendor.rs  -- 适配器 - 厂商
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::three::command::vendor::UpsertVendorCommand;
use cola_data::three::info::three_vendor::VendorInfo;
use cola_data::three::port::three_vendor::VendorPort;
use repo::three::pg::vendor_repo::VendorRepo;

//////

/// # [ADAPTER] - 厂商 端口适配器
pub struct VendorAdapter;

#[async_trait]
impl VendorPort for VendorAdapter {

    async fn upsert(&self, cmd: UpsertVendorCommand) -> anyhow::Result<VendorInfo> {
        let entity = VendorRepo::upsert(&cmd.code, &cmd.name, cmd.sort, cmd.status).await?;
        Ok(VendorInfo::from(entity))
    }

    async fn list(&self) -> anyhow::Result<Vec<VendorInfo>> {
        let entities = VendorRepo::list().await?;
        Ok(entities.into_iter().map(VendorInfo::from).collect())
    }

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<VendorInfo>> {
        let entity = VendorRepo::find_by_code(code).await?;
        Ok(entity.map(VendorInfo::from))
    }
}
