// repo_adapter/src/cola_market/goods/mod.rs
// 🔌 适配器 - MARKET - 商品 - mod
// 2026/6/18 10:45 Created.

////////

use async_trait::async_trait;
use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::goods::goods::GoodsInfo;
use port::cola_market::goods::GoodsPort;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod feed; // 流
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计
