// cola_im/src/api/chat.rs  -- IM - api - 聊天会话
// 2026/7/7 17:34

////////

use port::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_user::model::vo::video::VideoListResponse;
use crate::case;
use crate::case::chat::ChatCase;

////////

/// # [HOME API] -  聊天会话接口
pub struct ChatApi;

impl ChatApi {


    ////////

    /// # 1. [HANDLER] - 规则配置
    pub async fn handler_get_con(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<UserConfigInfo> {

        let uid = auth.uid;

        // Call Case:
        match ChatCase::case_get_con(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("New Videos Error: {:?}", e);

                AppData::err(5001, "获取用户配置失败", None)
            }
        }
    }

    ////////

    /// # 2. [HANDLER] - 添加
    pub async fn handler_add(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {


        let uid = auth.uid;

        match ChatCase::case_get_new_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("New Chat Error: {:?}", e);

                AppData::err(5001, "[✈️ API]: 添加新聊天会话失败", None)
            }
        }
    }

    ////////

    /// # 3. [HANDLER] - 关闭
    pub async fn handler_close(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;

        match ChatCase::case_get_hot_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),

            Err(e) => {
                tracing::error!("[✈️ API]: Close the Chat Error: {:?}", e);

                AppData::err(5001, "[API]: 关闭聊天会话失败", None)
            }
        }
    }

    ////////

    /// # 4. [HANDLER] - 删除
    pub async fn handler_delete(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match ChatCase::case_get_recommend_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("[✈️ API]: Delete Chat is Error: {:?}", e);
                AppData::err(5001, "删除聊天失败", None)
            }
        }
    }

    ////////

    /// # 5. [HANDLER] - 设置
    pub async fn handler_setting(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match ChatCase::case_get_city_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("[✈️ API]: Chat Setting is Error: {:?}", e);
                AppData::err(5001, "设置聊天失败", None)
            }
        }
    }

    ////////

    /// # 6. [HANDLER] - 置顶
    pub async fn handler_pin(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        let category_id = url.category_id;

        if category_id <= 0 {
            return AppData::err(4002, "参数错误：非法的 chat_id", None);
        }

        match ChatCase::case_get_category_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Category List Error: {:?}", e);
                AppData::err(5006, format!("置顶聊天失败: {}", e), None)
            }
        }
    }

    ////////

    /// # 7. [HANDLER] - 同步
    pub async fn handler_sync(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match ChatCase::case_get_featured_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Chat list sync Error: {:?}", e);
                AppData::err(5001, "聊天列表同步失败", None)
            }
        }
    }

    ////////

    /// # 8. [HANDLER] - 搜索
    pub async fn handler_get_search(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<VideoListResponse> {

        let uid = auth.uid;
        match ChatCase::case_get_keyword_list(uid, url, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => AppData::err(5006, format!("获取用户视频失败: {}", e), None),
        }
    }
}


//////// END
