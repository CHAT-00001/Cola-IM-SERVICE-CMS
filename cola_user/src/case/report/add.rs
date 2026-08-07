// cola_user/src/case/report/add2
// core - USER - case - report - add 添加 用例
// 2026/8/2 23:07 Created.

////////

use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::new::UserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_video::command::report::VideoReportCommand;
use tracing::info;

////////

/// # [ADD CASE] - 添加
/// * `desc`: `用户 举报 添加 用例`
pub struct UserReportAddCase;

impl UserReportAddCase {
    //

    ////////

    /// # 1. [CASE]] - 举报
    pub async fn case_add_report(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: VideoReportCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        ctx.video
            .report
            .add
            .save_report_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("🤐 [CASE]: ❌️ 保存举报记录失败: {}", e))?;

        info!("[🗣️ CASE]: ✅️ 保存举报记录成功: uid={}", uid);
        Ok(())
    }
}

//////// END
