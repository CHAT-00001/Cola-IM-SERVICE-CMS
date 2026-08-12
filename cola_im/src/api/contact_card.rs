// cola_im/src/api/card.rs  -- IM - api - 联系人 名片
// 2026/7/7 14:21

////////


// cola_video/src/api/home  -- 可乐短视频 - 接口层 - 主页
// 2026-04-16 08:00

////////

use port::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_user::info::config::UserConfigInfo;
use crate::case;
use crate::case::home::HomeCase;
use crate::model::vo::video::VideoListResponse;

////////

/// # [HOME API] -  主页 接口
pub struct HomeApi;

impl HomeApi {


    ////////

    /// # 1. [API HANDLER] - 规则配置
    pub async fn handler_get_con(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<UserConfigInfo> {

        let uid = auth.uid;

        // Call Case:
        match HomeCase::case_get_con(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);

                AppData::err(5001, "获取用户配置失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 最新
    pub async fn handler_get_new(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {


        let uid = auth.uid;

        match HomeCase::case_get_new_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);

                AppData::err(5001, "获取最新视频失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 热门
    pub async fn handler_get_hot(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;

        match HomeCase::case_get_hot_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("Recommend Error: {:?}", e);

                AppData::err(5001, "获取热门视频失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 推荐
    pub async fn handler_get_recommend(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match HomeCase::case_get_recommend_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Recommend Error: {:?}", e);
                AppData::err(5001, "获取推荐视频失败", None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 同城
    pub async fn handler_get_city(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match HomeCase::case_get_city_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Nearby Error: {:?}", e);
                AppData::err(5001, "获取同城视频失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 分类
    pub async fn handler_get_category(
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
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Category List Error: {:?}", e);
                AppData::err(5006, format!("获取分类视频失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 7. [API HANDLER] - 精选
    pub async fn handler_get_featured(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match HomeCase::case_get_featured_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Featured Error: {:?}", e);
                AppData::err(5001, "获取精选视频失败", None)
            }
        }
    }

    ////////

    /// # 8. [API HANDLER] - 搜索
    pub async fn handler_get_search(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match HomeCase::case_get_keyword_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5006, format!("获取用户视频失败: {}", e), None),
        }
    }
}


//////// END
