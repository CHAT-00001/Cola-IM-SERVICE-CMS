// port/src/cola_auth/session.rs
// 🕳️ 端口 - AUTH - port - 会话接口
// 2026/8/1 10:25

////////

use serde::{Deserialize, Serialize};

////////

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
