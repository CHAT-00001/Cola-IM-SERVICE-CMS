// repo_adapter/src/market/view/mod.rs
// 🔌 适配器 - MARKET - 商品浏览记录
// 2026/8/10 20:00 Updated.

////////

use port::market::view::GoodsViewPort;
use std::sync::Arc;

////////


pub mod add;     // 发布
pub mod active;  // 存活
pub mod del;     // 删除
pub mod get;     // 获取IDs
pub mod list;    // 浏览列表
pub mod manage;  // 管理
pub mod stat;    // 统计

////////

/// # [BUILD] - 构建 GOODS_VIEW Port
/// * `desc`: 商品浏览记录端口构造器
pub fn build_goods_view_port() -> GoodsViewPort {
    GoodsViewPort {
        active: Arc::new(active::AliveService),
        add: Arc::new(add::GoodsViewAddAdapter),
        del: Arc::new(del::GoodsViewDeleteAdapter),
        get: Arc::new(get::GoodsViewGetAdapter),
        list: Arc::new(list::ViewListService),
        manage: Arc::new(manage::GoodsViewManageAdapter),
        stat: Arc::new(stat::GoodsViewStatAdapter),
    }
}

//////// END
