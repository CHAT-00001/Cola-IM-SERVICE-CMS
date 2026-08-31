// cola_live/src/case/category/manage.rs
// 直播 - 用例层 - 分类 - 管理
// 2026/8/20 21:18 Created.

////////

use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::info::category::LiveCategoryInfo;
use port::app::ctx::AppContext;

////////

/// # 1. [CASE] - 直播分类管理
pub struct LiveCategoryManageCase;

impl LiveCategoryManageCase {
    /// # 1. [CASE] - 创建直播分类
    pub async fn create(
        uid: i64,
        command: LiveClassCommand,
        ctx: &AppContext,
    ) -> anyhow::Result<LiveCategoryInfo> {
        ctx.live.category.create(uid, command).await
    }

    /// # 2. [CASE] - 修改直播分类
    pub async fn edit(
        uid: i64,
        command: LiveClassCommand,
        ctx: &AppContext,
    ) -> anyhow::Result<LiveCategoryInfo> {
        ctx.live.category.edit(uid, command).await
    }

    /// # 3. [CASE] - 启用或禁用直播分类
    pub async fn change_status(
        uid: i64,
        id: i64,
        status: i16,
        ctx: &AppContext,
    ) -> anyhow::Result<LiveCategoryInfo> {
        ctx.live.category.change_status(uid, id, status).await
    }

    /// # 4. [CASE] - 删除直播分类
    pub async fn delete(uid: i64, id: i64, ctx: &AppContext) -> anyhow::Result<u64> {
        ctx.live.category.delete(uid, id).await
    }

    /// # 5. [CASE] - 查询直播分类列表
    pub async fn list(
        status: Option<i16>,
        limit: i64,
        offset: i64,
        ctx: &AppContext,
    ) -> anyhow::Result<Vec<LiveCategoryInfo>> {
        ctx.live.category.list(status, limit, offset).await
    }
}

//////// END
