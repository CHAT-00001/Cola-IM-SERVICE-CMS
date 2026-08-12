// cola_user/src/case/category/get.rs
// core - USER - case - 分类 - 获取
// 2026/8/4 01:28 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::app::page::PageInfo;
use cola_data::app::response::ListResponse;
use cola_data::cola_user::info::category::UserCategoryInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - [用户] 分类列表
pub struct UserCategoryGetCase;

impl UserCategoryGetCase {
    //

    ////////

    /// # 1. [CASE] - 最新的分类列表
    /// * `desc` 返回分类信息列表
    pub async fn case_get_new_list(
        uid: i64,
        limit: i64,  // 数量
        offset: i64, // 分页
        ctx: &AppContext,
    ) -> Result<ListResponse<UserCategoryInfo>, anyhow::Error> {
        // 1. 调用底层获取列表数据（多查一条用于判断是否有下一页，和前面的 Repo 逻辑保持一致）
        let mut infos = ctx
            .user
            .category
            .list
            .get_new_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 获取最新的分类列表失败: {}", e))?;

        // 2. 处理分页逻辑（判断是否有下一页）
        let mut has_next = false;
        if infos.len() > limit as usize {
            has_next = true;
            infos.pop(); // 弹出多查的那一条
        }

        // 3. 构建 PageInfo（由 offset/limit 反推页码）
        let page = if limit > 0 { offset / limit + 1 } else { 1 };
        let page_info = PageInfo {
            page,
            qty: limit,
            has_more: has_next,
        };

        // 4. 装载到泛型 ListResponse 中
        let response = ListResponse::new(infos, page_info);

        info!(
            "[🗣️ CASE]: ✅️ 最新的分类列表查询成功, uid={}, count={}",
            uid,
            response.list.len()
        );

        Ok(response)
    }
}

//////// END
