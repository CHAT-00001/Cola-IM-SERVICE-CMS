// repo_adapter/src/three/vendor.rs
// 🔌 适配器 - THREE - 厂商
// 2026/6/18 18:21

////////

use async_trait::async_trait;
use cola_data::cola_three::command::vendor::UpsertVendorCommand;
use cola_data::cola_three::info::vendor::VendorInfo;
use port::cola_three::vendor::VendorPort;
use repository::cola_three::pg::vendor_repo::VendorRepo;

////////

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

//////// END
