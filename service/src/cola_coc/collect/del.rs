// service/src/cola_video/collect/del.rs
// 👤 服务 - ▶ 可乐视频 - 收藏记录 - 删除
// 2026/8/10 00:36 Created.

////////

use repository::video::pg::collect::del::VideoCollectDelRepo;
use repository::video::pg::video::count::VideoCountRepo;

////////

/// # [DELETE SERVICE] - 删除
/// * `desc`: `▶ 可乐视频 - 收藏删除 SERVICE`
pub struct VideoCollectDelService;

impl VideoCollectDelService {
    //

    ////////

    /// # 1. [SERVICE] - 删除收藏记录
    /// * `DESC`: `删除单条收藏记录 + 同步视频收藏数 - 1`
    pub async fn single_delete(
        uid: i64,
        collect_id: i64, // 收藏 ID
    ) -> Result<(), anyhow::Error> {
        //--------

        // 1. 删除收藏记录
        let video_id = VideoCollectDelRepo::single_delete_by_id(collect_id).await?;

        //--------

        // 2. 更新收藏数量
        let update = VideoCountRepo::pg_update_video_collects(video_id as i64, -1).await?;

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 批量删除收藏记录 + 同步对应视频收藏数
    /// * `uid`: 用户ID (或操作者ID/管理员ID)
    pub async fn batch_delete(
        uid: i64,            // 操作者 ID
        collect_ids: &[i64], // 收藏 IDs
    ) -> Result<(), anyhow::Error> {
        // --------

        if collect_ids.is_empty() {
            return Ok(());
        }

        //-------

        // 1. 批量删除收藏记录
        let video_counts = VideoCollectDelRepo::batch_delete_by_ids(collect_ids).await?;

        Ok(())
    }
}

//////// END
