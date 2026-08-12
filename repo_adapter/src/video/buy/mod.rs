// repo_adapter/src/video/buy/mod.rs
// 🔌 适配器 - VIDEO - 购买 - 模块
// 2026/8/10 03:09 Created.

////////

use port::cola_video::buy::VideoBuyPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 COLLECT Port
/// * `desc`: 视频购买端口构造器
pub fn build_video_buy_port() -> VideoBuyPort {
    VideoBuyPort {
        add: Arc::new(add::BuyAddPortAdapter),
        check: Arc::new(check::BuyCheckPortAdapter),
        del: Arc::new(del::BuyDelPortAdapter),
        get: Arc::new(get::BuyGetPortAdapter),
        list: Arc::new(list::BuyListPortAdapter),
        manage: Arc::new(manage::BuyManagePortAdapter),
        stat: Arc::new(stat::BuyStatPortAdapter),
    }
}

//////// END
