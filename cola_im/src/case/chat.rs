// cola_im/src/case/chat.rs  -- IM - case - 聊天会话
// 2026/7/7 17:34

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [CHAT CASE] - 聊天会话 用例
pub struct ChatCase;

impl ChatCase {
    ////////
    pub async fn case_placeholder(
        _uid: i64,
        _url: ApiGatewayRequest,
        _ctx: &AppContext,
    ) -> Result<String> {
        Ok("ok".to_string())
    }
}

//////// END
