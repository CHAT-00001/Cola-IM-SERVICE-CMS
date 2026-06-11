// service/home.rs  -- 服务层 主页
// 2026/6/11 19:38

////////

use crate::video::pg::user::UserRepo;
use crate::video::pg::video::{VideoRepo};
use anyhow::Error;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::entity::collect::CollectEntity;
use cola_data::video::entity::video::VideoEntity;
use cola_data::video::info::video::VideoInfo;
use tracing::log;

////////

/// # [HOME SERVICE] - 主页 服务
pub struct VideoHomeService;

impl VideoHomeService {
    ////////

    ////////

    /// # 4. [SERVICE] - 保存收藏 + 更新计数
    pub async fn save_collect_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: CollectCommand,
    ) -> Result<CollectEntity, anyhow::Error> {
        // TODO: 替换为你底层的 CollectRepo 真实物理落库
        let collect_entity = CollectEntity::default();

        // 联动更新计数器：收藏的视频数量 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            // 收藏字段在第四位：publish, liked, total_favorited, collected
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户收藏计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        Ok(collect_entity)
    }

    ////////

    ////////

    /// # 7001. [SERVICE] - 获取用户发布的视频列表
    pub async fn find_user_publish_list(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 🌟 已修正：从 Repo 拿到物理表 Entity
        let entities = VideoRepo::find_new_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!("SERVICE: 获取用户{}发布的最新视频列表失败: {}", user_id, e)
            })?;

        // 🌟 已修正：加入调用构造函数转换
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////


    ////////

    /// # 10. [SERVICE] - 查找最新的视频列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取最新视频列表失败: {}", e))?;

        // 2. 🌟 拦截并就地转换为纯净的领域元数据 VideoInfo，彻底告别外泄
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取热门视频列表失败: {}", e))?;

        // 2. 🌟 拦截并就地脱敏、规范化，转换为纯净的领域元数据 VideoInfo
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 12. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取推荐视频列表失败: {}", e))?;

        // 2. 🌟 拦截并转换为纯净的领域元数据 VideoInfo
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 13. [SERVICE] - 查找同城的视频列表
    pub async fn find_city_video_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 🌟 已修正：返回值改为 Vec<VideoInfo>
        let entities = VideoRepo::find_nearby_list(lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取附近同城视频列表失败: {}", e))?;

        // 🌟 已修正：加入调用构造函数转换
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 14. [SERVICE] - 查找精选的视频列表
    pub async fn find_featured_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoRepo::find_featured_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取精选视频列表失败: {}", e))?;

        // 2. 🌟 拦截并就地转换为纯净的领域元数据 VideoInfo
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 15. [SERVICE] - 超级关键词检索
    pub async fn search_video_keyword_list(
        keyword: String,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 🌟 已修正：返回值改为 Vec<VideoInfo>
        let entities =
            VideoRepo::search_keyword_list(keyword, lat, lng, None, None, None, limit, offset)
                .await
                .map_err(|e| anyhow::anyhow!("SERVICE: 搜索视频失败: {}", e))?;

        // 🌟 已修正：加入调用构造函数转换，完美对接上层用例
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 16. [SERVICE] - 超级关键词检索
    /// * 根据用户IDs查找数据
    pub async fn find_video_by_user_ids(
        uids: Vec<i64>,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoRepo::find_list_by_uids(Option::from(uids), keyword, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 搜索视频失败: {}", e))?;

        // 2. 🌟 拦截并就地转换为纯净的领域元数据 VideoInfo
        let infos = entities
            .into_iter()
            .map(VideoInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }
}

//////// END