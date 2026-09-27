// cola_user/src/assembler/user.rs -- 用户 - assembler - 用户视图组装
// 2026/9/24 Created.

////////

use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::vo::user::UserVo;

////////

/// # 1. [ASSEMBLER] - 组装用户视图
/// * `desc`: 将用户信息与实时用户状态组装成通用 UserVo
pub fn build_user_vo(user_info: UserInfo) -> UserVo {
    UserVo::new(
        user_info.clone(),
        user_info.is_following,
        user_info.is_online,
        user_info.is_streaming,
    )
}

//////// END
