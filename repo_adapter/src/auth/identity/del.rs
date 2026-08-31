// repo_adapter/src/auth/file/del.rs
// 🔌 适配器 - AUTH - 身份识别 - 逻辑删除
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::auth::identity::del::IdentityDelPort;
use port::cola_video::comment::del::VideoCommentDelPort;

////////

/// # [DEL ADAPTER] - 发布
/// * `desc`: `AUTH - 身份逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct IdentityDelAdapter;

#[async_trait]
impl IdentityDelPort for IdentityDelAdapter {
    async fn single_delete(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
