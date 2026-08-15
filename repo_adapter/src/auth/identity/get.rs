// repo_adapter/src/auth/file/get.rs
// 🔌 适配器 - AUTH - 身份识别 - 获取IDs
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::identity::get::IdentityGetPort;
use port::cola_video::comment::get::VideoCommentGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `AUTH - 验证身份获取适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityGetAdapter;

#[async_trait]
impl IdentityGetPort for IdentityGetAdapter {
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
