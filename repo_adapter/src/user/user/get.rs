// repo_adapter/src/user/user/get.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::user::get::UserGetPort;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `用户获取服务`
pub struct UserGetAdapter;

// 构造实现
#[async_trait]
impl UserGetPort for UserGetAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 最新
    /// * `desc`: `保存新用户记录`

    async fn single_get_info(&self, id: i64) -> anyhow::Result<(UserInfo)> {
        todo!()
    }

    async fn batch_get_info(&self, ids: Vec<i64>) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }
}

//////// END
