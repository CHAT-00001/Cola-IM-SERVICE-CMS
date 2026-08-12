// cola_gis/src/api/get -- 可乐GIS - 接口层 浏览
// 2026-07-07

////////

use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use port::app::ctx::AppContext;
use crate::case::view::PoiViewCase;
use crate::model::vo::poi::PoiSingleResponse;

////////

/// # [API HANDLER] - 查看兴趣点详情
pub async fn handler_get_poi_detail(
    auth: AuthContext,
    url: ApiGatewayRequest,
    ctx: &AppContext,
) -> AppData<PoiSingleResponse> {

    let uid = auth.uid;

    match PoiViewCase::case_get_poi_detail(uid, url, ctx).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("POI_DETAIL_ERROR: {:?}", e);
            AppData::err(5001, "APP: 获取POI详情失败", None)
        }
    }
}

//////// END