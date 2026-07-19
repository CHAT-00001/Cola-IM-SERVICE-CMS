// repo/src/auth/service/session.rs  -- 仓储中心 - AUTH - 服务 - 会话
// 2026/06/05 06:25 by wx: cestbon10080

//////

use crate::auth::pg::session::SessionRepo;
use anyhow::{Result, anyhow};
use cola_data::auth::command::session::SessionCommand;
use cola_data::auth::entity::session::AuthSessionEntity; // 引入你的物理实体契约

pub struct SessionService;

impl SessionService {
    /// # 1. [SERVICE] - 保存/创建会话
    /// * 机制：严格对齐 AuthSessionEntity 的所有物理类型，干掉 E0308 编译错误
    pub async fn save_auth_session_info(uid: i64, cmd: SessionCommand) -> Result<i64> {
        // 🚀 【核心修复】在这里把 Command 的字段完美转换并塞给 Entity
        let entity = AuthSessionEntity {
            id: 0,
            user_id: uid,
            access_token: cmd.access_token,
            refresh_token: cmd.refresh_token,
            client_id: cmd.client_id,
            device_id: cmd.device_id,
            platform: 0,                         // i16: 0=未知平台
            access_expires_at: cmd.access_expires_at.timestamp(),
            refresh_expires_at: cmd.refresh_expires_at.timestamp(),
            last_active_at: cmd.last_active_at.timestamp(),

            status: 1,

            // 💡 2. 物理层是 DateTime<Utc> 时区对象：直接用 chrono 获取当前的标准时区实体
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // 调用你写的硬核 Repo 事务函数
        let new_id = SessionRepo::insert_session_with_kickout(entity)
            .await
            .map_err(|e| anyhow!("SERVICE: 创建会话并挤下线失败: {}", e))?;

        Ok(new_id)
    }

    ////////

    /// # 2. [SERVICE] - 检查会话状态 (通常给中间件鉴权使用)
    /// * 机制：未来改为旁路模式时，先去 Redis 查这个 token 存不存在。
    /// * 如果 Redis 穿透，下钻调用底层的 `find_active_session_by_token`，并回填 Redis。
    pub async fn check_auth_session_info(token: &str) -> Result<Option<AuthSessionEntity>> {
        if token.is_empty() {
            return Ok(None);
        }

        // 🚀 静态下探调用你的第 3 个 Repo 校验函数
        let session_opt = SessionRepo::find_active_session_by_token(token)
            .await
            .map_err(|e| anyhow!("SERVICE: 校验会话有效性失败: {}", e))?;

        Ok(session_opt)
    }

    ////////

    /// # 3. [SERVICE] - 删除会话信息 (主动注销退出)
    /// * 机制：未来改成旁路时，这里执行【双删策略】：Redis.del(token) -> Repo.logout_session
    pub async fn del_auth_session_info(
        session_id: &str, // 会话 ID
        device_id: &str,  // 设备 ID
    ) -> Result<u64> {
        if session_id.is_empty() {
            return Ok(0);
        }

        // 🚀 静态下探调用你的第 2 个 Repo 函数：把 status 改为 0
        let rows = SessionRepo::update_session_by_device_id(session_id, device_id)
            .await
            .map_err(|e| anyhow!("SERVICE: 注销会话失败: {}", e))?;

        Ok(rows)
    }

    ////////

    /// # 4. [SERVICE] - 退出登录（单设备下线）
    /// * 根据 user_id + device_id 唯一定位，将双 token 置为失效
    /// * 不影响该用户其他设备（多端登录友好）
    pub async fn logout_device(user_id: i64, device_id: &str) -> Result<u64> {
        if user_id <= 0 || device_id.is_empty() {
            return Ok(0);
        }

        let rows = SessionRepo::logout_session(user_id, device_id)
            .await
            .map_err(|e| anyhow!("SERVICE: 单设备退出失败: {}", e))?;

        Ok(rows)
    }

    ////////

    /// # 5. [SERVICE] - 获取用户当前所有在线设备
    pub async fn get_user_online_devices(user_id: i64) -> Result<Vec<AuthSessionEntity>> {
        if user_id <= 0 {
            return Ok(vec![]);
        }

        // 🚀 静态下探调用你的第 4 个 Repo 函数
        let list = SessionRepo::find_online_devices_by_uid(user_id)
            .await
            .map_err(|e| anyhow!("SERVICE: 获取在线设备列表失败: {}", e))?;

        Ok(list)
    }
}

///////// END
