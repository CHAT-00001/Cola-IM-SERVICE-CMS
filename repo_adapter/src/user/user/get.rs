// repo_adapter/src/user/user/get.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::user::get::UserGetPort;
use repository::user::pg::user::get::UserGetRepo;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `用户获取服务`
pub struct UserGetAdapter;

// 构造实现
#[async_trait]
impl UserGetPort for UserGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个获取
    async fn single_get_info(&self, user_id: i64) -> anyhow::Result<(UserInfo)> {
        let user = UserGetRepo::single_find_user_by_id(user_id)
            .await
            .map_err(|error| anyhow::anyhow!("[🤐 USER GET ADAPTER]: ❌️ 查询用户失败: {}", error))?
            .ok_or_else(|| anyhow::anyhow!("用户不存在: {}", user_id))?;

        Ok(user.into())
    }

    ////////

    /// # 2. [ADAPTER] - 批量获取
    async fn batch_get_infos(&self, user_ids: Vec<i64>) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }
}

//////// END
