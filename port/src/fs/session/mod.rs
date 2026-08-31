// port/src/auth/session/music.rs
// ⏩️ 端口 - AUTH - 会话 - 模块
// 2026/8/5 15:11 Created.

////////

use std::sync::Arc;

////////

use crate::auth::session::add::SessionAddPort;
use crate::auth::session::check::SessionCheckPort;
use crate::auth::session::del::SessionDelPort;
use crate::auth::session::get::SessionGetPort;
use crate::auth::session::list::SessionListPort;
use crate::auth::session::manage::SessionManagePort;
use crate::auth::session::stat::SessionStatPort;
use serde::{Deserialize, Serialize};

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [AUTH SESSION PORT]
/// * `desc`: `AUTH - 会话 Ports`
#[derive(Clone)]
pub struct AuthSessionPort {
    pub add: Arc<dyn SessionAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn SessionCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn SessionDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn SessionGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn SessionListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn SessionManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn SessionStatPort + Send + Sync + 'static>, // 统计
}

/// # [VO] - 会话验证结果（含用户信息）
/// * `desc`: `Token 验证通过后返回的可信会话数据`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionVerifyVo {
    /// 用户信息（供上层校验状态 / 提取 uid / roles）
    pub user_info: SessionUserInfo,
}

/// # [VO] - 会话验证 - 用户信息子集
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionUserInfo {
    /// 用户 ID
    pub uid: i64,
    /// 权限角色
    pub roles: Vec<String>,
    /// 账号状态（0 = 冻结）
    pub status: i16,
}

/// # [PORT] - 会话验证接口
/// * `desc`: `认证中心 Token 校验端口，由 repo_adapter 实现`
#[async_trait::async_trait]
pub trait SessionPort: Send + Sync {
    /// 1. 根据 access_token 获取可信会话数据
    /// * `None`: Token 无效或已过期
    async fn get_session(&self, token: &str) -> anyhow::Result<Option<SessionVerifyVo>>;
}

//////// END
