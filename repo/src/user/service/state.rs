// repo/src/user/service/state.rs  -- 仓储中心 - USER - 服务端口 - 用户状态
// 2026/6/5 00:38

////////


pub struct UserStateService;




impl UserStateService {
    // ... 之前写好的批量函数 ...

    /// # 检查用户是否真实合法存在
    pub async fn check_user_valid(uid: i64) -> Result<bool, anyhow::Error> {
        // 直接调 UserRepo::find_user_by_id(uid).await 看看是不是 Some 即可
        let user = crate::user::pg::user::UserRepo::find_user_by_id(uid).await?;
        Ok(user.is_some())
    }

    /// # 检查用户是否被拉黑封禁
    pub async fn check_user_banned(uid: i64) -> Result<bool, anyhow::Error> {
        // 先走 Redis 布隆过滤器或者查 user 表的 status 是否等于特定封禁码
        let user = crate::user::pg::user::UserRepo::find_user_by_id(uid).await?;
        Ok(user.map(|u| u.status.unwrap_or(1) == 0).unwrap_or(false)) // 假设 status = 0 为封禁
    }
}