// repo_adapter/src/market/goods/mod.rs
// 🔌 适配器 - MARKET - 商品
// 2026/8/10 20:00 Updated.

////////

use port::market::goods::GoodsPort;
use std::sync::Arc;

////////

// 模块占位符 - 待实现具体操作
pub mod add;    // 发布
pub mod check;  // 检查
pub mod del;    // 删除
pub mod feed;   // 流
pub mod get;    // 获取
pub mod list;   // 列表
pub mod manage; // 管理
pub mod stat;   // 统计

////////

/// # [BUILD] - 构建 GOODS Port
/// * `desc`: 商品端口构造器
pub fn build_goods_port() -> GoodsPort {
    GoodsPort {
        add: Arc::new(add::GoodsAddAdapter),
        check: Arc::new(check::GoodsCheckAdapter),
        delete: Arc::new(del::GoodsDeleteAdapter),
        get: Arc::new(get::GoodsGetAdapter),
        list: Arc::new(list::GoodsListAdapter),
        manage: Arc::new(manage::GoodsManageAdapter),
        stat: Arc::new(stat::GoodsStatAdapter),
    }
}

//////// END
