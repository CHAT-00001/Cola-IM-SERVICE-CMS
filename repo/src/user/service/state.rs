// /state.rs  -- 用户 状态 服务
// 2026/6/5 00:38

////////

use crate::user::pg::state::UserStateRepo;
use cola_data::user::command::user::UserCommand;
use cola_data::user::info::user::UserInfo;

////////

/// # [SERVICE] - 用户 状态 服务
pub struct UserStateService;

// 构造函数
impl UserStateService {
    ////////

    ////////

    /// # 1. [SERVICE] - 检查用户存在
    pub async fn check_user_valid(
        user_id: i64, // 用户 ID
    ) -> Result<bool, anyhow::Error> {
        // Call Repo
        let user = crate::user::pg::user::UserRepo::find_user_by_id(user_id).await?;
        Ok(user.is_some())
    }

    ////////

    /// # 2. [SERVICE] - 检查用户封禁
    pub async fn check_user_banned(uid: i64) -> Result<bool, anyhow::Error> {
        // 先走 Redis 布隆过滤器或者查 user 表的 status 是否等于特定封禁码
        let user = crate::user::pg::user::UserRepo::find_user_by_id(uid).await?;
        Ok(user.map(|u| u.status.unwrap_or(1) == 0).unwrap_or(false)) // 假设 status = 0 为封禁
    }

    ////////

    /// # 3. [SERVICE] - 查找和创建 Orchestrator
    /// * 返回 (用户信息, 是否为新用户)
    pub async fn upsert_user_by_phone(phone_no: String) -> Result<(UserInfo, bool), anyhow::Error> {
        // 1. 查找现有用户
        if let Some(user) = UserStateRepo::find_user_by_phone(&phone_no).await? {
            // 存在，返回已有用户信息，is_new_user 为 false
            return Ok((user.into(), false));
        }

        // 2. 如果不存在，执行创建逻辑
        let cmd = UserCommand::new_with_phone(phone_no);
        let entity = cmd.new();

        // 3. 入库
        let saved_entity = UserStateRepo::save_user(entity).await?;

        // 4. 返回新用户信息，is_new_user 为 true
        Ok((saved_entity.into(), true))
    }
}

//////// END
