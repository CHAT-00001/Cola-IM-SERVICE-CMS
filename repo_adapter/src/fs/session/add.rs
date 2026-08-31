// repo_adapter/src/auth/sessiony/add.rs  --
// 🔌 适配器 - AUTH - SESSION - 发布 服务
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::auth::session::add::SessionAddPort;
use port::cola_video::comment::add::VideoCommentAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `AUTH - 验证会话发布服务`
#[derive(Debug, Default, Clone)]
pub struct SessionAddAdapter;

#[async_trait]
impl SessionAddPort for SessionAddAdapter {
    async fn save_session(
        &self,
        uid: i64,
        user_id: i64,
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
