// cola_live/src/case/category/add.rs
// 直播 - 用例层 - 分类 - 发布用例编排
// 2026/8/20 20:12 Created.

////////

use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::info::category::LiveCategoryInfo;
use port::app::ctx::AppContext;

/// # [ADD CASE] - 发布用例
/// * `desc`: `管理员发布直播分类用例`
pub struct LiveCategoryAddCase;
impl LiveCategoryAddCase {
    //

    ////////
    pub async fn case_create_category(
        uid: i64,
        cmd: LiveClassCommand, // 命令
        ctx: &AppContext,      // 全局上下文
    ) -> anyhow::Result<LiveCategoryInfo> {
        ctx.live.category.create(uid, cmd).await
    }
}

//////// END
