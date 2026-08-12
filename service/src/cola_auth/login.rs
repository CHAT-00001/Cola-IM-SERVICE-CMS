// service/src/auth/login.rs
// 服务 - 可乐验证 - 登录 - 会话
// 2026/6/9 08:37 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_auth::command::session::SessionCommand;
use cola_data::cola_auth::entity::session::AuthSessionEntity;
use repository::cola_auth::pg::session::SessionRepo;

/////////

/// # [ADD SERVICE] - 登录服务
/// * `desc`:  `用户登录创建会话`
pub struct LoginService;

impl LoginService {
    ////////

    /// # 1. [SERVICE] - 保存/创建会话
    pub async fn save_auth_session_info(user_id: i64, cmd: SessionCommand) -> Result<i64> {
        // 🚀 对齐最新的 SessionCommand：只保留核心字段
        let entity = AuthSessionEntity {
            id: 0,
            user_id,
            access_token: cmd.access_token,
            refresh_token: cmd.refresh_token,
            client_id: "".to_string(),
            device_id: "".to_string(),
            platform: 0, // i16: 0=未知平台
            access_expires_at: cmd.access_expires_at.timestamp(),
            refresh_expires_at: cmd.refresh_expires_at.timestamp(),
            last_active_at: cmd.last_active_at.timestamp(),
            status: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 调用 Repo 层执行入库（记得在 Repo 更新入库 SQL 以匹配现在的字段）
        let new_id = SessionRepo::insert_session_with_kickout(entity)
            .await
            .map_err(|e| anyhow!("SERVICE: 创建会话失败: {}", e))?;

        Ok(new_id)
    }

    ////////

    /// # 2. [SERVICE] - 检查会话状态
    pub async fn check_auth_session_info(token: &str) -> Result<Option<AuthSessionEntity>> {
        if token.is_empty() {
            return Ok(None);
        }

        SessionRepo::find_active_session_by_token(token)
            .await
            .map_err(|e| anyhow!("SERVICE: 校验会话失败: {}", e))
    }

    /////////

    /// # 3. [SERVICE] - 删除会话信息 (主动注销)
    pub async fn del_auth_session_info(user_id: i64, token: &str) -> Result<u64> {
        // 注销通常基于 user_id 和 token，而不是之前的 session_id/device_id
        SessionRepo::logout_session(user_id, token)
            .await
            .map_err(|e| anyhow!("SERVICE: 注销会话失败: {}", e))
    }

    ////////

    /// # 4. [SERVICE] - 获取用户当前所有在线设备
    pub async fn get_user_online_devices(user_id: i64) -> Result<Vec<AuthSessionEntity>> {
        if user_id <= 0 {
            return Ok(vec![]);
        }

        SessionRepo::find_online_devices_by_uid(user_id)
            .await
            .map_err(|e| anyhow!("SERVICE: 获取列表失败: {}", e))
    }
}

//////// END
