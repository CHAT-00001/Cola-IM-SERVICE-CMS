// repo_adapter/src/auth/file/add.rs  --
// 🔌 适配器 - AUTH - 身份识别 - 发布
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::identity::add::IdentityAddPort;
use port::cola_video::comment::add::VideoCommentAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `AUTH - 身份识别发布适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityAddAdapter;

#[async_trait]
impl IdentityAddPort for IdentityAddAdapter {
    async fn send_comment(
        &self,
        uid: i64,
        video_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn edit_comment(
        &self,
        uid: i64,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}

//////// END
