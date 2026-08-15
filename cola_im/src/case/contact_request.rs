// cola_im/src/case/contact_request.rs  -- IM - api - 联系人 添加请求
// 2026/7/7 14:22

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [REQUEST CASE] - 联系人请求 用例
pub struct RequestCase;

impl RequestCase {
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