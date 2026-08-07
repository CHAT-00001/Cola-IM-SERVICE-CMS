// core - 视频 - api - 主页
// 2026-04-16 08:00

////////

use crate::case;
use crate::case::home::HomeCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::video::info::video::VideoListResponse;

////////

/// # [HOME API] -  主页 接口
pub struct HomeApi;

impl HomeApi {
    //

    ////////

    /// # 1. [API HANDLER] - 最新
    pub async fn home_new(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match HomeCase::case_get_new_list(uid, url, ctx).await {
            // Ok
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Videos Susee!");
                AppData::ok(resp)
            }

            // Fail
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ New Videos Error: {:?}", e);

                AppData::err(5001, "获取最新视频失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 热门
    pub async fn home_hot(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;

        match HomeCase::case_get_hot_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Hot Videos Susee!");
                AppData::ok(resp)
            }

            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Hot Videos Error: {:?}", e);

                AppData::err(5001, "获取热门视频失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 推荐
    pub async fn home_recommend(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        match HomeCase::case_get_recommend_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Recommend Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Recommend Videos Error: {:?}", e);
                AppData::err(5001, "获取推荐视频失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 同城
    pub async fn home_city(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        match HomeCase::case_get_city_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get City Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ City Videos Error: {:?}", e);
                AppData::err(5001, "获取同城视频失败", None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 分类
    pub async fn home_category(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        let category_id = url.category_id;

        if category_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 category_id", None);
        }

        match HomeCase::case_get_category_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Category Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Category Videos Error: {:?}", e);
                AppData::err(5006, format!("获取分类视频失败: {}", e), None)
            }
        }
    }

    /// # 6. [API HANDLER] - 频道
    pub async fn home_channel(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        let channel_id = url.category_id;

        if channel_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 channel_id", None);
        }

        match HomeCase::case_get_category_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Channel Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Channel Videos Error: {:?}", e);
                AppData::err(5006, format!("获取频道视频失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 7. [API HANDLER] - 精选
    pub async fn home_featured(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        match HomeCase::case_get_featured_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Featured Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Featured Videos Error: {:?}", e);
                AppData::err(5001, "获取精选视频失败", None)
            }
        }
    }

    ////////

    /// # 8. [API HANDLER] - 搜索
    pub async fn home_search(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {
        let uid = auth.uid;
        match HomeCase::case_get_keyword_list(uid, url, ctx).await {
            Ok(resp) => {
                tracing::info!("[🗣️ API] - ✅️ Get Search Videos Susee!");
                AppData::ok(resp)
            }
            Err(e) => {
                tracing::error!("[🤐 API] - ❌️ Search Videos Error: {:?}", e);
                AppData::err(5006, format!("获取用户视频失败: {}", e), None)
            }
        }
    }
}

//////// END