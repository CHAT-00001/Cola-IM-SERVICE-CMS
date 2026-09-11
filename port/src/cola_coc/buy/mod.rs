// port/src/cola_coc/buy/mod.rs -- 端口 - COC - 购买 - mod
// 2026/8/5 15:53 Created.

////////

use crate::cola_video::buy::add::VideoBuyAddPort;
use crate::cola_video::buy::check::VideoBuyCheckPort;
use crate::cola_video::buy::del::VideoBuyDelPort;
use crate::cola_video::buy::get::VideoBuyGetPort;
use crate::cola_video::buy::list::VideoBuyListPort;
use crate::cola_video::buy::manage::VideoBuyManagePort;
use crate::cola_video::buy::stat::VideoBuyStatPort;
use std::sync::Arc;

////////

pub mod add;
pub mod alive;
pub mod check;
pub mod del;
pub mod get;
pub mod list;
pub mod manage;
pub mod order;
pub mod stat;

////////

/// # [COC BUY PORTS]
/// * `desc`: `COLA COC - 视频购买 Ports`
#[derive(Clone)]
pub struct VideoBuyPort {
    pub add: Arc<dyn VideoBuyAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn VideoBuyCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn VideoBuyDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn VideoBuyGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn VideoBuyListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn VideoBuyManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn VideoBuyStatPort + Send + Sync + 'static>, // 统计
}

//////// END
