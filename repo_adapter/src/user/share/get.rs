// repo_adapter/src/user/share/get.rs  --
// 🔌 插头 - 可乐用户 - 分享 - 获取
// 2026/8/8 12:45 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::share::get::UserShareGetPort;

////////

/// # [SHARE ADAPTER] - 获取
/// * `desc`: `USER - 主页分享查询适配器`
pub struct ShareGetAdapter;

#[async_trait]
impl UserShareGetPort for ShareGetAdapter {
    async fn get_share_ids(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_share_me_ids(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
    //

    ////////
}

//////// END
