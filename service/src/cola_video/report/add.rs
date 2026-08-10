// service/src/cola_video/report/add.rs
// 👤 服务 - ▶ 可乐视频 - 举报 - 发布
// 2026/6/10 19:27

////////

use cola_data::cola_video::command::report::VideoReportCommand;
use cola_data::cola_video::entity::comment::VideoCommentEntity;
use tracing::log;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `▶ 可乐视频 - 视频举报发布服务`
pub struct VideoReportAddService;

// 构造实现
impl VideoReportAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存举报 + 更新计数 (纯静态函数适配器)
    pub async fn save_comment_and_update_count(
        uid: i64,
        cmd: VideoReportCommand,
    ) -> Result<Vec<VideoCommentEntity>, anyhow::Error> {
        // TODO: 替换为你底层的 CommentRepo 真实物理落库
        // 这里返回一组数据，模拟老代码中用 pop 提取实体的行为
        let mock_entity = VideoCommentEntity::default();
        let saved_list = vec![mock_entity];

        // 异步更新用户的评论/互动相关计数（如果后续要在用户表增加 comment_count，在这里改 0）
        let _async_uid = uid;
        tokio::spawn(async move {
            // 目前短视频用户表没有细分评论数，预留
        });

        Ok(saved_list)
    }

    ////////

    /// # 3. [SERVICE] - 检查视频状态
    pub async fn check_video_state(video_id: i64) -> Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 3. [SERVICE] - 检查视频状态
    // 假设你的项目使用 anyhow 或自定义错误类型
    pub async fn check_user_state(video_id: i64) -> Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }

    ////////
}

//////// END
