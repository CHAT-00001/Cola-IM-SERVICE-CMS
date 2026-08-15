// repo_adapter/src/auth/file/manage.rs
// 🔌 适配器 - AUTH - 身份识别 - 管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::identity::manage::IdentityManagePort;
use port::cola_video::comment::manage::VideoCommentManagePort;

////////

/// # [ADD ADAPTER] - 管理
/// * `desc`: `AUTH - 验证身份管理适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityManageAdapter;

#[async_trait]
impl IdentityManagePort for IdentityManageAdapter {
    async fn admin_list(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}
