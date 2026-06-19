// cola_user/src/api/add.rs  -- USER - 接口层 - 添加
// 2026/4/12 14:45

//////

use crate::case::add::AddCase;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::user::command::new::UserCommand;
use cola_data::user::command::update_user::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;

//////

/// # [ADD CASE] - 用户 接口
pub struct AddApi;

// 构造函数
impl AddApi {
    ////////

    /// # 1. [API HANDLER] - 新建用户（网关调用）
    pub async fn handler_get_new(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<UserInfo> {
        let uid = auth.uid;

        // 构建用户命令，从 url/body 等提取参数（此处为示例，后续可完善解析）
        let cmd = UserCommand::default();

        // 执行核心用户创建逻辑
        match AddCase::case_add_new_user(uid, cmd, ctx.clone()).await {
            Ok(resp) => AppData::ok(resp).with_msg("新建用户成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("新建用户失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑用户资料
    pub async fn handler_edit_user(
        auth: AuthContext,
        cmd: UpdateUserCommand,
        ctx: &AppContext,
    ) -> AppData<UserInfo> {
        let uid = auth.uid;

        // 执行核心编辑用户资料逻辑
        match AddCase::case_update_profile(uid, cmd, ctx.clone()).await {
            Ok(resp) => AppData::ok(resp).with_msg("修改用户资料成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("修改用户资料失败: {:?}", e), None),
        }
    }
}

//////// END
