// repo_adapter/src/market/cate/mod.rs -- 适配器 - MARKET - 商品分类 - mod
// 2026/8/10 20:00 Updated.

////////

use port::market::cate::GoodsCatePort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 CATE Port
/// * `desc`: 商品分类端口构造器
pub fn build_cate_port() -> GoodsCatePort {
    GoodsCatePort {
        add: Arc::new(add::CateAddAdapter),
        check: Arc::new(check::CateCheckAdapter),
        del: Arc::new(del::CateDelAdapter),
        get: Arc::new(get::CateGetAdapter),
        list: Arc::new(list::CateListAdapter),
        manage: Arc::new(manage::CateManageAdapter),
        stat: Arc::new(stat::CateStatAdapter),
    }
}

//////// END
