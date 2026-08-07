// repo_adapter/src/cola_three/server_type.rs  -- 适配器 - THREE - 服务类型
// 2026/6/18 18:19

////////

use async_trait::async_trait;
use cola_data::cola_three::command::category::ThreeServerTypeCommand;
use cola_data::cola_three::info::server_type::{ServerTypeInfo};
use cola_data::cola_three::port::category::TypePort;
use repository::cola_three::pg::three_type::{ServerTypeRepo};

////////

/// # [ADAPTER] - 服务类型 端口适配器
pub struct TypeAdapter;

#[async_trait]
impl TypePort for TypeAdapter {

    ////////

    async fn upsert(&self, cmd: ThreeServerTypeCommand) -> anyhow::Result<ServerTypeInfo> {
        let entity = ServerTypeRepo::upsert(&cmd.code, &cmd.name, cmd.sort, cmd.status).await?;
        Ok(ServerTypeInfo::from(entity))
    }

    ////////

    async fn list(&self) -> anyhow::Result<Vec<ServerTypeInfo>> {
        let entities = ServerTypeRepo::list().await?;
        Ok(entities.into_iter().map(ServerTypeInfo::from).collect())
    }

    ////////

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<ServerTypeInfo>> {
        let entity = ServerTypeRepo::find_by_code(code).await?;
        Ok(entity.map(ServerTypeInfo::from))
    }
}

//////// END