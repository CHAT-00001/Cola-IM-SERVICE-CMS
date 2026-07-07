// repo_adapter/src/three/three_biz_binding.rs  -- 适配器 - 绑定
// 2026/6/30

////////
/// 
use async_trait::async_trait;
use cola_data::three::command::three_biz_binding::UpsertBindingCommand;
use cola_data::three::info::three_biz_binding::BindingInfo;
use cola_data::three::port::three_biz_binding::BindingPort;
use repo::three::pg::binding_repo::BindingRepo;

////////

/// # [ADAPTER] - 业务绑定 端口适配器
pub struct BindingAdapter;

#[async_trait]
impl BindingPort for BindingAdapter {

    ////////

    /// 1. #[ADAPTER] - 插入或更新
    async fn upsert(&self, cmd: UpsertBindingCommand) -> anyhow::Result<BindingInfo> {
        let entity = BindingRepo::upsert(cmd.three_config_id, &cmd.biz_module, &cmd.biz_type, cmd.status).await?;
        Ok(BindingInfo::from(entity))
    }

    ////////

    /// 2. #[ADAPTER] - 列表
    async fn list(&self) -> anyhow::Result<Vec<BindingInfo>> {
        let entities = BindingRepo::list().await?;
        Ok(entities.into_iter().map(BindingInfo::from).collect())
    }

    /////////

    
    /// 3. #[ADAPTER] - 按业务模块和类型查询
    async fn find_by_biz(&self, biz_module: &str, biz_type: &str) -> anyhow::Result<Option<BindingInfo>> {
        let entity = BindingRepo::find_by_biz(biz_module, biz_type).await?;
        Ok(entity.map(BindingInfo::from))
    }
}

//////// END