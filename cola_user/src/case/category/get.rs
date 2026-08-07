// user/src/case/category/get.rs
// 用户 - case - 分类 - 获取
// 2026/8/4 01:28 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::app::page::PageInfo;
use cola_data::app::response::ListResponse;
use cola_data::user::info::category::UserCategoryInfo;
use tracing::info;

////////

pub struct UserCategoryGetCase;

impl UserCategoryGetCase {
    //

    ////////

    /// # 1. [CASE] - 获取最新列表
    /// * `desc` 返回分类信息列表
    pub async fn case_get_new_list(
        uid: i64,
        offset: i64,
        limit: i64,
        ctx: &AppContext,
    ) -> Result<ListResponse<UserCategoryInfo>, anyhow::Error> {
        // 1. 调用底层获取列表数据（多查一条用于判断是否有下一页，和前面的 Repo 逻辑保持一致）
        let mut infos = ctx
            .user
            .category
            .get_new_list(uid, limit, offset)
            .await
            .map_err(|e| anyhow!("[CASE]: ❌️ 获取分类列表失败: {}", e))?;

        // 2. 处理分页逻辑（判断是否有下一页）
        let mut has_next = false;
        if infos.len() > limit as usize {
            has_next = true;
            infos.pop(); // 弹出多查的那一条
        }

        // 3. 构建 PageInfo（根据你的 PageInfo 结构适配，这里假设支持类似字段或构造方法）
        let page_info = PageInfo {
            page,
            qty,
            has_more,
            ..Default::default()
        };

        // 4. 装载到泛型 ListResponse 中
        let response = ListResponse::new(infos, page_info);

        info!(
            "[CASE]: 分类列表查询成功, uid={}, count={}",
            uid,
            response.list.len()
        );

        Ok(response)
    }
}

//////// END
