// repo_adapter/src/three/three_type.rs  -- 适配器 - 服务类型
// 2026/6/18

//////

use async_trait::async_trait;
use cola_data::three::command::three_type::UpsertTypeCommand;
use cola_data::three::info::three_type::TypeInfo;
use cola_data::three::port::three_type::TypePort;
use repo::three::pg::three_type::TypeRepo;

//////

/// # [ADAPTER] - 服务类型 端口适配器
pub struct TypeAdapter;

#[async_trait]
impl TypePort for TypeAdapter {

    async fn upsert(&self, cmd: UpsertTypeCommand) -> anyhow::Result<TypeInfo> {
        let entity = TypeRepo::upsert(&cmd.code, &cmd.name, cmd.sort, cmd.status).await?;
        Ok(TypeInfo::from(entity))
    }

    async fn list(&self) -> anyhow::Result<Vec<TypeInfo>> {
        let entities = TypeRepo::list().await?;
        Ok(entities.into_iter().map(TypeInfo::from).collect())
    }

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<TypeInfo>> {
        let entity = TypeRepo::find_by_code(code).await?;
        Ok(entity.map(TypeInfo::from))
    }
}
