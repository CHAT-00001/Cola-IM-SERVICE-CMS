// repo_adapter/src/auth/identity/list.rs
// 🔌 适配器 - AUTH - 身份识别 - 评论列表
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::identity::list::IdentityListPort;
use port::cola_video::comment::list::VideoCommentListPort;

////////

/// # [LIAT ADAPTER] - 列表
/// * `desc`: `AUTH - 验证身份列表适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityListAdapter;

#[async_trait]
impl IdentityListPort for IdentityListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}
