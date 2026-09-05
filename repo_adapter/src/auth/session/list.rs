// repo_adapter/src/auth/session/list.rs -- 适配器 - AUTH - SESSION - 列表适配器
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::session::list::SessionListPort;
use port::cola_video::comment::list::VideoCommentListPort;

////////

/// # [LIST ADAPTER] - 发布
/// * `desc`: `AUTH - 验证会话发布服务`
#[derive(Debug, Default, Clone)]
pub struct SessionListAdapter;

#[async_trait]
impl SessionListPort for SessionListAdapter {
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
