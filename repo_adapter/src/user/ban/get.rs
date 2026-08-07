// repo_adapter/src/user/ban/get.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 获取服务
// 2026/8/7 06:09 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::ban::get::BanGetPort;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `用户封禁获取服务`
pub struct BanGetService;

// 构造实现
#[async_trait]
impl BanGetPort for BanGetService {
    //

    ////////

    /// # 1. [SERVICE] - 获取
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `风控触发 + 审核人员发布`
    async fn get_my_black_ids(
        &self,
        uid: i64,
        id: i64, // UID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 获取
    /// * `desc`: `管理员封禁/改权限`
    async fn get_he_black_ids(
        &self,
        uid: i64, // UID
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # 3. [SERVICE] - 获取
    /// * `desc`: `管理员封禁/改权限`
    async fn get_black_me_ids(
        &self,
        uid: i64, // UID
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # 4. [SERVICE] - 获取
    /// * `desc`: `管理员封禁/改权限`
    async fn get_black_he_ids(
        &self,
        uid: i64, // UID
        id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
