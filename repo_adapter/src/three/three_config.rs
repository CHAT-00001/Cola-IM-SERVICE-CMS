// repo_adapter/src/three/three_config.rs  -- 适配器 - 配置
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::three::command::three_config::UpsertConfigCommand;
use cola_data::three::info::three_config::ConfigInfo;
use cola_data::three::port::three_config::ConfigPort;
use repo::three::pg::three_config::ConfigRepo;

//////

/// # [ADAPTER] - 配置 端口适配器
pub struct ConfigAdapter;

#[async_trait]
impl ConfigPort for ConfigAdapter {

    async fn upsert(&self, cmd: UpsertConfigCommand) -> anyhow::Result<ConfigInfo> {
        let entity = if let Some(id) = cmd.id {
            ConfigRepo::update(
                id, cmd.type_id, cmd.vendor_id, &cmd.name, &cmd.bucket,
                &cmd.access_key, &cmd.secret_key, &cmd.endpoint, &cmd.region,
                cmd.config_json.as_ref(), cmd.remark.as_deref(), cmd.status,
            ).await?
        } else {
            ConfigRepo::insert(
                cmd.type_id, cmd.vendor_id, &cmd.name, &cmd.bucket,
                &cmd.access_key, &cmd.secret_key, &cmd.endpoint, &cmd.region,
                cmd.config_json.as_ref(), cmd.remark.as_deref(), cmd.status,
            ).await?
        };
        Ok(ConfigInfo::from(entity))
    }

    async fn list_by_type(&self, type_id: i64) -> anyhow::Result<Vec<ConfigInfo>> {
        let entities = ConfigRepo::list_by_type(type_id).await?;
        Ok(entities.into_iter().map(ConfigInfo::from).collect())
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<ConfigInfo>> {
        let entity = ConfigRepo::find_by_id(id).await?;
        Ok(entity.map(ConfigInfo::from))
    }

    async fn find_binded(&self, biz_module: &str, biz_type: &str) -> anyhow::Result<Option<ConfigInfo>> {
        let entity = ConfigRepo::find_binded(biz_module, biz_type).await?;
        Ok(entity.map(ConfigInfo::from))
    }
}
