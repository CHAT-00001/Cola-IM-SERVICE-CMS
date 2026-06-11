// service/feed.rs  - 服务层 浏览
// 2026/6/10 17:44

////////

use crate::video::pg::video::{ VideoRepo};
use crate::video::pg::view::VideoViewRepo;
use cola_data::video::entity::video::VideoEntity;
use cola_data::video::info::video::VideoInfo;
use tracing::log;

////////

/// # [SERVICE] - 视频服务
pub struct ViewService;

impl ViewService {
    // * --------
    ////////

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
        let db_videos = VideoViewRepo::find_all_batch_ids(&ids)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 批量获取视频 entity 列表失败: {}", e))?;

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
            .ok_or_else(|| anyhow::anyhow!("BIZ: 视频 {} 不存在", id))
    }

    ////////

    /// # 3. [SERVICE] - 遍历uids批量获取视频信息
    /// * `desc` : (关注/朋友) 的feed流
    pub async fn batch_uids_get_videos_infos(
        uids: Vec<i64>,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        if uids.is_empty() {
            return Ok(vec![]);
        }

        // 1. 从 DB 捞出原始 Entity 列表
        let db_videos = VideoViewRepo::pg_batch_uids_find_list(uids, keyword, offset, limit)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 批量获取视频 entity 列表失败: {}", e))?;

        // 2. 纯内存无损转换
        let video_infos = db_videos.into_iter().map(VideoInfo::from_entity).collect();

        Ok(video_infos)
    }

    ////////

    /// # 4. [SERVICE] - 根据uid获取视频信息
    /// * `desc` : (用户主页) 的feed流
    pub async fn get_videos_infos_by_uid(
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        let uid = user_id;

        // 1. 从 DB 捞出原始 Entity 列表
        let db_videos = VideoViewRepo::pg_find_new_list_by_uid(uid, keyword, offset, limit)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 批量获取视频 entity 列表失败: {}", e))?;

        // 2. 纯内存无损转换
        let video_infos = db_videos.into_iter().map(VideoInfo::from_entity).collect();

        Ok(video_infos)
    }

    ////////

    /// # 10. [SERVICE] - 查找最新的视频列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取热门视频列表失败: {}", e))
    }

}

//////// END
