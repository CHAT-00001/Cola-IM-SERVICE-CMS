// service/add.rs  -- 服务层 添加
// 2026/6/9 19:19

////////

use crate::pg_pool;
use crate::video::pg::add::AddRepository;
use crate::video::pg::user::UserRepo;
use crate::video::pg::video::VideoRepo;
use crate::video::redis::video::VideoCache;
use crate::video::redis::visited::VisitedCache;
use anyhow::Result;
use app_config::DbService;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::entity::video::VideoEntity;
use cola_data::video::info::video::VideoInfo;
use std::collections::HashMap;
use tracing::log;

////////

/// # [ADD SERVICE] - 添加 服务
pub struct AddService;

// 构造函数
impl AddService {
    // * --------
    // * --------

    ////////

    /// # 1. [SERVICE] - 保存视频 + 更新计数 (纯静态函数适配器)
    pub async fn save_video_and_update_count(
        uid: i64,          // 用户 ID 核心参数
        cmd: VideoCommand, // 视频创建命令
        visibility: i16,   // 风控可见性
    ) -> Result<VideoInfo, anyhow::Error> {
        // 🌟 返回值无缝升级为 VideoInfo
        // 1. Call Repo
        // * 调用底层仓储 - 保存视频并直接返回插入后的物理实体数据
        let video_entity = VideoRepo::save_video_by_uid(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 写入视频主表失败: {}", e))?;

        // 2. Call Repo
        // * 联动更新计数器：发布视频数 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户视频计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        // 3. 🌟 核心升级：就地消化物理 Entity，转换为纯净领域元数据
        let video_info = VideoInfo::from_entity(video_entity);

        Ok(video_info)
    }

    ////////

    /// # 2. [SERVICE] - 编辑视频
    pub async fn edit_video(
        uid: i64,          // 用户 ID 核心参数
        cmd: VideoCommand, // 视频创建命令
        visibility: i16,   // 风控可见性
    ) -> Result<VideoInfo, anyhow::Error> {
        // 🌟 返回值无缝升级为 VideoInfo
        // 1. Call Repo
        // * 调用底层仓储 - 保存视频并直接返回插入后的物理实体数据
        let video_entity = VideoRepo::save_video_by_uid(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 写入视频主表失败: {}", e))?;

        // 2. Call Repo
        // * 联动更新计数器：发布视频数 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户视频计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        // 3. 🌟 核心升级：就地消化物理 Entity，转换为纯净领域元数据
        let video_info = VideoInfo::from_entity(video_entity);

        Ok(video_info)
    }

    ////////

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
    ////////

    /// # 10. [SERVICE] - 删除一个视频
    pub async fn del_one_video_and_update_count(video_id: i64) -> Result<bool, anyhow::Error> {
        // Call Repo
        match AddRepository::pg_delete_video_by_id(video_id).await {
            Ok(_) => Ok(true),                          // 删除成功
            Err(sqlx::Error::RowNotFound) => Ok(false), // 视频不存在或已删除
            Err(e) => Err(anyhow::anyhow!("SERVICE: 删除视频失败: {}", e)),
        }
    }

    ////////

    /// # 10. [SERVICE] - 批量删除视频
    pub async fn del_many_video_and_update_count(
        video_ids: Vec<i64>,
    ) -> Result<bool, anyhow::Error> {
        // Call Repo
        match AddRepository::pg_delete_video_by_ids(video_ids).await {
            Ok(_) => Ok(true),                          // 删除成功
            Err(sqlx::Error::RowNotFound) => Ok(false), // 视频不存在或已删除
            Err(e) => Err(anyhow::anyhow!("SERVICE: 批量删除视频失败: {}", e)),
        }
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

    ////////

    /// # 12. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取推荐视频列表失败: {}", e))
    }

    ////////
}

//////// END
