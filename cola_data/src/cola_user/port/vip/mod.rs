// cola_user/port/vip/mod.rs
// 用户 - port - 贵宾 - 模块
// 2026/8/5 23:33 Created.

////////

use crate::cola_user::port::vip::add::VipAddPort;
use crate::cola_user::port::vip::check::VipCheckPort;
use crate::cola_user::port::vip::del::VipDelPort;
use crate::cola_user::port::vip::get::VipGetPort;
use crate::cola_user::port::vip::list::VipListPort;
use crate::cola_user::port::vip::manage::VipManagePort;
use std::sync::Arc;

pub mod add; // 开通
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理

////////

/// # [VIP PORTS]
/// * `desc`: `用户贵宾端口`
#[derive(Clone)]
pub struct VipPort {
    pub add: Arc<dyn VipAddPort + Send + Sync + 'static>,
    pub check: Arc<dyn VipCheckPort + Send + Sync + 'static>,
    pub del: Arc<dyn VipDelPort + Send + Sync + 'static>,
    pub get: Arc<dyn VipGetPort + Send + Sync + 'static>,
    pub list: Arc<dyn VipListPort + Send + Sync + 'static>,
    pub manage: Arc<dyn VipManagePort + Send + Sync + 'static>,
}

//////// END
