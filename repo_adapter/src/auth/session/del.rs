// repo_adapter/src/auth/session/del.rs
// 🔌 适配器 - AUTH - SESSION - 逻辑删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::auth::session::del::SessionDelPort;
use port::cola_video::comment::del::VideoCommentDelPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `AUTH - 验证会话发布服务`
#[derive(Debug, Default, Clone)]
pub struct SessionDelAdapter;

#[async_trait]
impl SessionDelPort for SessionDelAdapter {
    async fn single_delete(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
