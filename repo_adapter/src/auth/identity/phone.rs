// repo_adapter/src/auth/identity/phone.rs -- ADAPTER - AUTH - 身份 - 手机身份绑定
// 2026/8/20 Created.

////////

use async_trait::async_trait;
use port::auth::identity::phone::PhoneIdentityPort;
use repository::auth::pg::identity::phone::PhoneIdentityRepo;
use tracing::{error, info};

////////

/// # [ADAPTER] - 手机身份绑定适配器
#[derive(Debug, Default, Clone)]
pub struct PhoneIdentityAdapter;

////////

#[async_trait]
impl PhoneIdentityPort for PhoneIdentityAdapter {
    /// # 1. [ADAPTER] - 查询手机号绑定用户
    /// * `desc`: `转发到 AUTH identity 仓储`
    async fn find_user_id_by_phone(&self, phone: &str) -> anyhow::Result<Option<i64>> {
        PhoneIdentityRepo::find_user_id_by_phone(phone)
            .await
            .map_err(|error| {
                error!(
                    "[🤐 ADAPTER] - ❌️ 手机身份查询失败: phone={}, error={}",
                    phone, error
                );
                error
            })
    }

    /// # 2. [ADAPTER] - 绑定手机号
    /// * `desc`: `将手机号绑定到用户`
    async fn bind_phone(&self, user_id: i64, phone: &str) -> anyhow::Result<()> {
        PhoneIdentityRepo::bind_phone(user_id, phone)
            .await
            .map(|_| {
                info!("[🗣️ ADAPTER] - ✅️ 手机身份绑定成功: user_id={}", user_id);
            })
            .map_err(|error| {
                error!(
                    "[🤐 ADAPTER] - ❌️ 手机身份绑定失败: user_id={}, error={}",
                    user_id, error
                );
                error
            })
    }
}

//////// END
