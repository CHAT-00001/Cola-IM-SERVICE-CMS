// cola_live/src/case/stream/add.rs -- LIVE - case - 直播场次 - 开播与停播
// 2026/8/21 09:42 Created.

////////

use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_data::cola_live::info::record::LiveRecordInfo;
use port::app::ctx::AppContext;

////////

/// # 1. [CASE] - 直播场次开播流程
pub struct LiveStreamAddCase;

impl LiveStreamAddCase {
    /// # 1. [CASE] - 开播
    /// * `desc`: `校验可信会话、用户状态、开播权限、直播间后创建场次`
    pub async fn start(
        auth: &AuthContext,
        command: LiveRecordCommand,
        ctx: &AppContext,
    ) -> anyhow::Result<LiveRecordInfo> {
        if auth.uid <= 0 || auth.is_anonymous {
            anyhow::bail!("登录会话无效");
        }
        if command.room_id <= 0 {
            anyhow::bail!("直播间ID无效");
        }
        let user = ctx.user.profile.get.single_get_info(auth.uid).await?;
        if user.id != auth.uid || user.status != 1 {
            anyhow::bail!("用户状态异常，无法开播");
        }
        ctx.live
            .stream
            .check
            .can_start(auth.uid, command.room_id)
            .await?;
        ctx.live.stream.add.start(auth.uid, command).await
    }

    /// # 2. [CASE] - 停播
    /// * `desc`: `只允许主播关闭自己的直播场次`
    pub async fn stop(auth: &AuthContext, record_id: i64, ctx: &AppContext) -> anyhow::Result<()> {
        if auth.uid <= 0 || auth.is_anonymous {
            anyhow::bail!("登录会话无效");
        }
        if record_id <= 0 {
            anyhow::bail!("直播场次ID无效");
        }
        ctx.live.stream.manage.stop(auth.uid, record_id).await
    }
}

//////// END
