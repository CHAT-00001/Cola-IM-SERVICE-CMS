// cola_im/src/case/card.rs  -- IM - api - 联系人 名片
// 2026/7/7 14:21

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [CARD CASE] - 名片 用例
pub struct CardCase;

impl CardCase {
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