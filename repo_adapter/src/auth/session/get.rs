// repo_adapter/src/auth/session/get.rs
// 🔌 适配器 - AUTH - SESSION - 获取IDs
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::session::get::SessionGetPort;
use port::cola_video::comment::get::VideoCommentGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `可乐视频 - 视频评论发布服务`
#[derive(Debug, Default, Clone)]
pub struct SessionGetAdapter;

#[async_trait]
impl SessionGetPort for SessionGetAdapter {
    async fn get_comment_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }

    async fn get_comment_by_video(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }
}
