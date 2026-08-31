// port/src/cola_video/video/mod.rs
// ⏩️ 端口 - MARKET - 商品分类 - music
// 2026/8/5 15:57 Created.

////////

use crate::market::cate::add::CateAddPort;
use crate::market::cate::check::CateCheckPort;
use crate::market::cate::del::CateDeletePort;
use crate::market::cate::get::CateGetPort;
use crate::market::cate::list::CateListPort;
use crate::market::cate::manage::CateManagePort;
use crate::market::cate::stat::CateStatPort;
use std::sync::Arc;
////////

pub mod add; // 发布
pub mod check; // 检查
pub mod count; // 计数
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [VIDEO PORTS] - 商品分类
/// * `desc`: `MARKET - 分类 Ports`
#[derive(Clone)]
pub struct GoodsCatePort {
    pub add: Arc<dyn CateAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CateCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn CateDeletePort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn CateGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn CateListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn CateManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn CateStatPort + Send + Sync + 'static>, // 管理
}

//////// END
