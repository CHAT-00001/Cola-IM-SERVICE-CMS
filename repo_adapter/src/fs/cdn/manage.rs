// repo_adapter/src/auth/file/manage.rs
// 🔌 适配器 - AUTH - 身份识别 - 管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::fs::cdn::manage::CdnManagePort;

////////

/// # [ADD ADAPTER] - 管理
/// * `desc`: `FS - CDN管理适配器`
#[derive(Debug, Default, Clone)]
pub struct CdnManageAdapter;

#[async_trait]
impl CdnManagePort for CdnManageAdapter {
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
