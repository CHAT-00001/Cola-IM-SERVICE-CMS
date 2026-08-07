// repo_adapter/src/user/ban/manage.rs
// 🔌 适配器 - 可乐用户 - 封禁 - 管理服务
// 2026/8/7 06:05 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::ban::manage::BanManagePort;

////////

/// # [MANAGE SERVICE] - 管理
/// * `desc`: `管理员管理服务`
pub struct BanManageService;

// 构造实现
#[async_trait]
impl BanManagePort for BanManageService {
    //

    ////////

    /// # 1. [SERVICE] - 发布
    /// * `desc`: `管理员封禁/改权限`
    /// * `warning`: `⚠️ 预设接口`
    async fn get_appeal_list(&self, uid: i64, limit: i64, offset: i64) -> Result<(i64, Vec<i64>)> {
        todo!()
    }

    async fn set_deny_appeal(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }

    async fn set_reject_appeal(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }

    async fn set_close_appeal(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }

    async fn set_reassign_appeal(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }

    async fn set_final_review(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }

    async fn set_unban(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END
