// cola_user/src/assembler/user.rs -- 用户 - assembler - 用户视图组装
// 2026/9/24 Created.

////////

use cola_data::cola_user::info::profile::ProfileInfo;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::response::profile::{
    ProfileContentStats, ProfileCount, ProfileRelation, ProfileSingleResponse, ProfileSocialStats,
    ProfileVo,
};

////////

pub const USER_HOME_APP_ID: &str = "short-video";
pub const USER_HOME_BIZ_ID: &str = "user-home";

////////

/// # [HELPER] - 拼接用户主页资源 CDN 地址
/// * `desc`: 兼容绝对地址和相对路径
pub fn resolve_profile_cdn_url(path: &str, cdn_domain: &str) -> String {
    if path.is_empty()
        || path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("//")
    {
        return path.to_string();
    }

    format!(
        "{}/{}",
        cdn_domain.trim().trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

////////

/// # [ASSEMBLER] - 用户主页组装输入
/// * `desc`: Case 查询完成后交给 Assembler 的纯数据
#[derive(Debug, Clone, Default)]
pub struct UserHomeAssembleInput {
    pub user_info: UserInfo,
    pub profile_info: Option<ProfileInfo>,
    pub social: ProfileSocialStats,
    pub content: ProfileContentStats,
    pub count: ProfileCount,
    pub relation: ProfileRelation,
    pub cdn_domain: String,
}

////////

/// # 1. [ASSEMBLER] - 构造用户主页响应
/// * `desc`: `COLA USER - Build User Home Profile RESPONSE DATA.`
pub fn build_user_home_response(mut input: UserHomeAssembleInput) -> ProfileSingleResponse {
    input.user_info.avatar_url =
        resolve_profile_cdn_url(&input.user_info.avatar_url, &input.cdn_domain);
    input.user_info.bg_img = resolve_profile_cdn_url(&input.user_info.bg_img, &input.cdn_domain);

    ProfileSingleResponse {
        info: ProfileVo::combine(
            USER_HOME_APP_ID.to_string(),
            USER_HOME_BIZ_ID.to_string(),
            input.user_info,
            input.profile_info,
            input.social,
            input.content,
            input.count,
            input.relation,
        ),
    }
}

//////// END
