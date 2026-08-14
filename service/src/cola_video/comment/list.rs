// servicey/src/cola_video/identity/list.rs
// 👤 服务 - ▶ 可乐评论  - 评论 - 前台列表
// 2026/8/8 00:33 Created.

////////

use anyhow::Error;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use repository::cola_video::pg::comment::list::VideoCommentListRepo;
use tracing::log;

////////

/// # [COMMENT LIST SERVICE] - 前台列表
/// * `desc`: `▶ 可乐评论 - 👤 评论评论列表服务`
pub struct VideoCommentListService;

impl VideoCommentListService {
    //

    ////////

    /// # 1. [SERVICE] - 用户的
    /// * `desc`: `根据用户 ID - 获取评论记录列表`
    pub async fn get_new_list_by_user_id(
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        // 🌟 已修正：从 Repo 拿到物理表 Entity
        let entities = VideoCommentListRepo::find_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "[🤐 LIST SERVICE]: - ❌️ 获取用户ID [{}] 发布的最新评论列表失败: {}",
                    user_id,
                    e
                )
            })?;

        // 🌟 已修正：加入调用构造函数转换
        let infos = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 1. [SERVICE] - 评论的
    /// * `desc`: `根据评论 ID - 获取评论记录列表`
    pub async fn get_new_list_by_video_id(
        video_id: i64, // 评论 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        // 🌟 已修正：从 Repo 拿到物理表 Entity
        let entities = VideoCommentListRepo::find_list_by_video_id(video_id, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!(
                    "[🤐 LIST SERVICE]: - ❌️ 获取评论ID [{}] 下的最新评论列表失败: {}",
                    video_id,
                    e
                )
            })?;

        // 🌟 已修正：加入调用构造函数转换
        let infos = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 10. [SERVICE] - 查找最新的评论列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoCommentListRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 LIST SERVICE]: - ❌️ 获取最新评论列表失败: {}", e))?;

        // 2. 🌟 拦截并就地转换为纯净的领域元数据 VideoCommentInfo，彻底告别外泄
        let infos = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    ////////

    /// # 11. [SERVICE] - 查找热门的评论列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        // 1. 从 Repo 拿到物理表 Entity 列表
        let entities = VideoCommentListRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 LIST SERVICE]: - ❌️ 获取热门评论列表失败: {}", e))?;

        // 2. 🌟 拦截并就地脱敏、规范化，转换为纯净的领域元数据 VideoCommentInfo
        let infos = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }

    ////////

    /// # 13. [SERVICE] - 查找同城的评论列表
    pub async fn find_city_video_list(
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        // 🌟 已修正：返回值改为 Vec<VideoCommentInfo>
        let entities = VideoCommentListRepo::find_nearby_list(lat, lng, limit, offset)
            .await
            .map_err(|e| {
                anyhow::anyhow!("[🤐 LIST SERVICE]: - ❌️ 获取附近同城评论列表失败: {}", e)
            })?;

        // 🌟 已修正：加入调用构造函数转换
        let infos = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect::<Vec<_>>();

        Ok(infos)
    }
}

//////// END
