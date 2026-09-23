// cola_user/src/api/user/history.rs -- 用户资料及历史接口
// 2026/9/23 Created.

////////

use crate::case::user::history::UserHistoryCase;
use cola_data::app::data::AppData;
use cola_data::app::page::PageInfo;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::info::user_history::UserHistoryInfo;
use port::app::ctx::AppContext;
use serde::{Deserialize, Serialize};

////////

/// # [API] - 用户资料与头像/昵称历史
pub struct UserHistoryApi;

/// # [REQUEST] - 用户资料更新请求
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub signature: Option<String>,
    pub avatar: Option<String>,
    pub avatar_thumb: Option<String>,
    pub bg_img: Option<String>,
    pub sns_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<i64>,
    pub lat: Option<String>,
    pub lng: Option<String>,
}

impl From<UpdateProfileRequest> for UpdateUserCommand {
    fn from(value: UpdateProfileRequest) -> Self {
        Self {
            nickname: value.nickname,
            signature: value.signature,
            avatar: value.avatar,
            avatar_thumb: value.avatar_thumb,
            bg_img: value.bg_img,
            sns_url: value.sns_url,
            email: value.email,
            phone: value.phone,
            birthday: value.birthday,
            lat: value.lat,
            lng: value.lng,
            updated_at: Some(chrono::Utc::now()),
        }
    }
}

/// # [RESPONSE] - 用户历史分页
#[derive(Debug, Clone, Serialize)]
pub struct UserHistoryPage {
    pub list: Vec<UserHistoryInfo>,
    pub page: PageInfo,
}

impl UserHistoryApi {
    pub async fn update(
        uid: i64,
        request: UpdateProfileRequest,
        ctx: &AppContext,
    ) -> AppData<UserInfo> {
        match UserHistoryCase::update(uid, request.into(), ctx).await {
            Ok(info) => AppData::ok(info).with_msg("用户资料更新成功"),
            Err(error) => AppData::err(5001, format!("用户资料更新失败: {error}"), None),
        }
    }

    pub async fn list(
        uid: i64,
        is_avatar: bool,
        query: cola_data::app::query::ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<UserHistoryPage> {
        match UserHistoryCase::list(
            uid,
            is_avatar,
            query.page.unwrap_or(1),
            query.qty.unwrap_or(10),
            ctx,
        )
        .await
        {
            Ok((list, page)) => AppData::ok(UserHistoryPage { list, page }),
            Err(error) => AppData::err(5001, format!("历史记录查询失败: {error}"), None),
        }
    }

    pub async fn activate(
        uid: i64,
        is_avatar: bool,
        id: i64,
        ctx: &AppContext,
    ) -> AppData<UserInfo> {
        match UserHistoryCase::activate(uid, is_avatar, id, ctx).await {
            Ok(info) => AppData::ok(info).with_msg("历史记录启用成功"),
            Err(error) => AppData::err(5001, format!("历史记录启用失败: {error}"), None),
        }
    }

    pub async fn delete(uid: i64, is_avatar: bool, id: i64, ctx: &AppContext) -> AppData<()> {
        match UserHistoryCase::delete(uid, is_avatar, id, ctx).await {
            Ok(()) => AppData::empty(),
            Err(error) => AppData::err(5001, format!("历史记录删除失败: {error}"), None),
        }
    }
}

//////// END
