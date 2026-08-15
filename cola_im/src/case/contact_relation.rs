// cola_im/src/case/relation.rs  -- IM - api - 联系人 关系
// 2026/7/7 14:22

////////

use anyhow::Result;
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [RELATION CASE] - 联系人关系 用例
pub struct RelationCase;

impl RelationCase {
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
