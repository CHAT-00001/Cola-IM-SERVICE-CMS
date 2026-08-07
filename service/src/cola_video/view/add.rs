// servicey/src/cola_video/view/add.rs
// 👤 服务 - ▶ 可乐视频 - 浏览 - 发布
// 2026/6/10 17:44

////////

use cola_data::cola_video::info::video::VideoInfo;
use tracing::log;
use cola_data::cola_video::entity::video::video::VideoEntity;
use repository::cola_video::pg::video::get::VideoGetRepo;

////////

/// # [VIEW ADD SERVICE] - 发布
/// * `desc`: `▶ 可乐视频 - 👤 视频浏览发布服务`
pub struct VideoViewAddService;

// 构造实现

impl VideoViewAddService {
    // 💡

    ////////

    /// # 1. [SERVICE] - 批量获取视频信息
    /// `desc`: (全站核心，点赞/收藏/历史/详情页全走这里)
    pub async fn batch_get_videos_infos(
        ids: Vec<i64>
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        // 1. 从 DB 捞出原始 Entity 列表
        let db_videos = VideoGetRepo::find_list_batch_by_ids(&ids)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 批量获取视频 handler 列表失败: {}", e))?;

        // 2. 纯内存无损转换
        let video_infos = db_videos.into_iter().map(VideoInfo::from_entity).collect();

        Ok(video_infos)
    }

    /// # 2. [SERVICE] - 根据id单个获取视频信息
    /// `desc`: (语义化包装，底层直接寄生在批量函数上)
    pub async fn get_one_video_info(id: i64) -> Result<VideoInfo, anyhow::Error> {
        // 直接把单 ID 打包成 Vec 喂给批量函数
        let mut infos = Self::batch_get_videos_infos(vec![id]).await?;

        // 捞得到就吐出来，捞不到就报 404
        infos
            .pop()
            .ok_or_else(|| anyhow::anyhow!("[👤 SERVICE]:: 视频 {} 不存在", id))
    }

    ////////


}

//////// END
