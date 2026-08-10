// repo_adapter/src/video/collect/manage.rs
// 🔌 适配器 - ▶ 可乐视频 - Collect - Manage 实现
// 2026/8/9 01:32 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::info::collect::VideoCollectInfo;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use cola_data::cola_video::port::collect::manage::CollectManagePort;
use repository::cola_video::pg::collect::manage::VideoCollectManageRepo;

////////

/// # [MANAGE ADAPTER] - 收藏
/// * `desc`: `收藏记录管理适配器`
#[derive(Debug, Default, Clone)]
pub struct CollectManageAdapter;

// 构造实现
#[async_trait]
impl CollectManagePort for CollectManageAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 管理员 - 视频的
    /// * `desc`: `管理员视角的记录列表`
    async fn get_collect_record_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> Result<(Vec<VideoCollectInfo>)> {
        // 1. Call REPOSITOTY .. -- 获取视频收藏记录
        let entities = VideoCollectManageRepo::get_collect_record_list_by_video_id(
            video_id, // 视频 ID
            limit,    // 数量
            offset,   // 页码
        )
        .await;

        // 2. 使用 iterator 转换
        let infos: Vec<VideoCollectInfo> = entities
            .into_iter()
            .map(VideoCollectInfo::from) // 如果实现了 From<VideoCollectEntity>
            .collect();

        Ok(infos)
    }

    /// # 2. [SERVICE] - 管理员 - 用户的
    /// * `desc`: `管理员视角的记录列表`
    async fn get_collect_record_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<VideoCollectInfo>)> {
        // 1. Call REPOSITOTY .. -- 获取视频收藏记录
        let entities = VideoCollectManageRepo::get_collect_record_list_by_video_id(
            user_id, // 用户 ID
            limit,   // 数量
            offset,  // 页码
        )
        .await;

        // 2. 使用 iterator 转换
        let infos: Vec<VideoCollectInfo> = entities
            .into_iter()
            .map(VideoCollectInfo::from) // 如果实现了 From<VideoCollectEntity>
            .collect();

        Ok(infos)
    }

    ////////

    /// # 2. [SERVICE] - 管理员列表
    /// * `desc`: `管理员视角的记录列表`
    async fn edit_comment_record(
        &self,
        comment_id: i64,
        cmd: CommentCommand,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    ////////

    /// # 3. [SERVICE] - 管理员列表
    /// * `desc`: `管理员视角的记录列表`
    async fn del_comment_record(&self, comment_id: i64) -> Result<()> {
        todo!()
    }

    ////////

    /// # 4. [SERVICE] - 管理员列表
    /// * `desc`: `管理员视角的记录列表`
    async fn del_comments_record(&self, comment_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END
