// port/src/cola_live/category.rs -- 端口 - LIVE - 分类 - mod
// 2026/8/20 21:12 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::info::category::LiveCategoryInfo;

////////

/// # 1. [PORT] - 直播分类管理端口
/// * `desc`: `管理员创建、修改、状态切换、删除和查询直播分类`
#[async_trait]
pub trait LiveCategoryPort: Send + Sync + 'static {
    async fn create(&self, uid: i64, command: LiveClassCommand)
    -> anyhow::Result<LiveCategoryInfo>;
    async fn edit(&self, uid: i64, command: LiveClassCommand) -> anyhow::Result<LiveCategoryInfo>;
    async fn change_status(
        &self,
        uid: i64,
        id: i64,
        status: i16,
    ) -> anyhow::Result<LiveCategoryInfo>;
    async fn delete(&self, uid: i64, id: i64) -> anyhow::Result<u64>;
    async fn get(&self, id: i64) -> anyhow::Result<Option<LiveCategoryInfo>>;
    async fn list(
        &self,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<LiveCategoryInfo>>;
}

//////// END
