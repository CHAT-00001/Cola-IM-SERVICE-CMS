// repo_adapter/src/market/collect/mod.rs -- 适配器 - MARKET - 商品收藏 - mod
// 2026/8/10 20:00 Updated.

////////

use port::market::collect::GoodsCollectPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 计数

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 商品收藏夹端口构造器
pub fn build_collect_port() -> GoodsCollectPort {
    GoodsCollectPort {
        add: Arc::new(add::GoodsCollectAddAdapter),
        check: Arc::new(check::GoodsCollectCheckAdapter),
        del: Arc::new(del::GoodsCollectDelAdapter),
        get: Arc::new(get::GoodsCollectGetAdapter),
        list: Arc::new(list::GoodsCollectListAdapter),
        manage: Arc::new(manage::GoodsCollectManageAdapter),
        stat: Arc::new(stat::GoodsCollectStatAdapter),
    }
}

//////// END
