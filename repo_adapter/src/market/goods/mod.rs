// repo_adapter/src/cola_market/goods/mod.rs
// 适配器 - 市场 - 商品 - 模块(原子化解耦)
// 2026/6/18
// 2026/8/6 解耦: add / list / del / get / manage / check

////////

use async_trait::async_trait;
use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::goods::goods::GoodsInfo;
use cola_data::cola_market::port::goods::GoodsPort;

pub mod add;
pub mod list;
pub mod del;
pub mod get;
pub mod manage;
pub mod check;

////////

/// # [ADAPTER] - 商品 端口适配器
pub struct GoodsAdapter;

#[async_trait]
impl GoodsPort for GoodsAdapter {

    ////////

    /// # 1. [ADAPTER] - 保存商品
    async fn save_goods(
        &self,
        uid: i64, // 用户ID
        cmd: GoodsCommand, // 商品命令
    ) -> anyhow::Result<()> {
        add::save(uid, cmd).await
    }

    ////////

    /// # 2. [ADAPTER] - 编辑商品
    async fn update_goods(
        &self,
        uid: i64, // 用户ID
        goods_id: i64, // 商品ID
        cmd: GoodsCommand, // 商品命令
    ) -> anyhow::Result<()> {
        add::update(uid, goods_id, cmd).await
    }

    ////////

    /// # 3. [ADAPTER] - 修改状态(上架/下架)
    async fn change_status(
        &self,
        uid: i64, // 用户ID
        goods_id: i64, // 商品ID
    ) -> anyhow::Result<()> {
        manage::change_status(uid, goods_id).await
    }

    ////////

    /// # 4. [ADAPTER] - 删除商品
    async fn delete_goods(
        &self,
        uid: i64, // 用户ID
        goods_id: i64, // 商品ID
    ) -> anyhow::Result<()> {
        del::soft_delete_single(uid, goods_id).await
    }

    ////////

    /// # 5. [ADAPTER] - 获取我的商品列表
    async fn get_address_by_user_id(
        &self,
        uid: i64, // 用户ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
    ) -> anyhow::Result<Vec<GoodsInfo>> {
        list::get_my_list(uid, offset, limit).await
    }

    ////////

    /// # 6. [ADAPTER] - 浏览商品详情
    async fn view_goods_by_id(
        &self,
        uid: i64, // 用户ID
        goods_id: i64, // 商品ID
    ) -> anyhow::Result<GoodsInfo> {
        get::get_detail(uid, goods_id).await
    }

    ////////

    /// # 7. [ADAPTER] - 删除用户所有地址
    async fn delete_address_by_user_id(
        &self,
        uid: i64, // 用户ID
        user_id: i64, // 目标用户ID
    ) -> anyhow::Result<()> {
        del::soft_delete_by_user(uid, user_id).await
    }
}

//////// END