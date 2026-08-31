// repo_adapter/src/live/category.rs -- 适配器 - LIVE - 直播分类
// 2026/8/20 21:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::info::category::LiveCategoryInfo;
use port::cola_live::category::LiveCategoryPort;
use repository::cola_live::pg::category_repo::LiveCategoryRepo;

////////

/// # 1. [ADAPTER] - 直播分类适配器
#[derive(Debug, Default, Clone)]
pub struct LiveCategoryAdapter;

#[async_trait]
impl LiveCategoryPort for LiveCategoryAdapter {
    async fn create(
        &self,
        uid: i64,
        command: LiveClassCommand,
    ) -> anyhow::Result<LiveCategoryInfo> {
        Ok(LiveCategoryRepo::create(uid, command).await?.into())
    }

    async fn edit(&self, uid: i64, command: LiveClassCommand) -> anyhow::Result<LiveCategoryInfo> {
        Ok(LiveCategoryRepo::edit(uid, command).await?.into())
    }

    async fn change_status(
        &self,
        uid: i64,
        id: i64,
        status: i16,
    ) -> anyhow::Result<LiveCategoryInfo> {
        Ok(LiveCategoryRepo::change_status(uid, id, status)
            .await?
            .into())
    }

    async fn delete(&self, _uid: i64, id: i64) -> anyhow::Result<u64> {
        LiveCategoryRepo::delete(id).await
    }

    async fn get(&self, id: i64) -> anyhow::Result<Option<LiveCategoryInfo>> {
        Ok(LiveCategoryRepo::get(id).await?.map(Into::into))
    }

    async fn list(
        &self,
        status: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<LiveCategoryInfo>> {
        Ok(LiveCategoryRepo::list(status, limit, offset)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

//////// END
