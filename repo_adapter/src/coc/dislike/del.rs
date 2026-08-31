// repo_adapter/src/cola_video/dislike/del.rs
// 🔌 插头 - 可乐视频 - 不喜欢 - 删除
// 2026/8/6 18:58 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::dislike::del::VideoDislikeDelPort;

////////

/// # [DELETE ADAPTER] - dislike del
/// * `desc`: `▶ 视频 - 不喜欢记录删除适配器`
#[derive(Debug, Default, Clone)]
pub struct dislikedelPortAdapter;

#[async_trait]
impl VideoDislikeDelPort for dislikedelPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个
    async fn single_soft_del_record(
        &self,
        uid: i64,
        dislike_id: i64, // 不喜欢 ID
    ) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量
    async fn batch_soft_del_record(
        &self,
        uid: i64,
        dislike_ids: Vec<i64>, // 不喜欢 IDs
    ) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 用户的
    async fn delete_dislike_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 视频的
    async fn delete_dislike_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> Result<(u16)> {
        todo!()
    }
}

//////// END
