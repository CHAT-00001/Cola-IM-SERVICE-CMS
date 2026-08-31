// service/src/cola_video/video/add.rs
// 👤 服务 - VIDEO - 视频 - 发布
// 2026/8/2 12:39 Created.

////////

use anyhow::Result;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::video::VideoInfo;
use repository::video::pg::user::count::UserCountRepo;
use repository::video::pg::video::add::VideoAddRepository;
use repository::video::pg::video::home::VideoRepo;
use std::collections::HashMap;
use tracing::log;

////////

/// # [ADD SERVICE] - 视频发布
/// * `desc`: `用户视频发布服务`
pub struct VideoAddService;

// 构造函数
impl VideoAddService {
    //

    ////////

    /// # 1. [SERVICE] - 新增
    /// * `desc`: `保存视频 + 更新计数 (纯静态函数适配器)`
    pub async fn save_video_and_update_count(
        uid: i64,             // 用户 ID 核心参数
        cmd: VideoNewCommand, // 视频创建命令
        visibility: i16,      // 风控可见性
    ) -> Result<VideoInfo, anyhow::Error> {
        let video_entity = VideoAddRepository::pg_save_video_by_uid(uid, cmd, visibility as i64, 4)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 写入视频主表失败: {}", e))?;

        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserCountRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户视频计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        let video_info = VideoInfo::from_entity(video_entity);

        Ok(video_info)
    }

    ////////

    /// # 2. [SERVICE] - 修改内容
    /// * `desc`: `编辑视频 + 更新缓存`
    pub async fn edit_content(
        uid: i64,                // 用户 ID 核心参数
        cmd: VideoUpdateCommand, // 视频创建命令
        visibility: i16,         // 风控可见性
    ) -> Result<VideoInfo, anyhow::Error> {
        let video_entity = VideoAddRepository::update_content_by_video_id(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 SERVICE]: ❌️ 写入视频主表失败: {}", e))?;

        let video_info = VideoInfo::from_entity(video_entity);

        Ok(video_info)
    }

    ////////

    /// # 3. [SERVICE] - 修改权限
    /// * `desc`: `修改视频权限 + 更新缓存`
    pub async fn change_permission(
        uid: i64,                          // 用户 ID 核心参数
        cmd: VideoUpdatePermissionCommand, // 视频创建命令
    ) -> Result<VideoInfo, anyhow::Error> {
        let video_entity = VideoAddRepository::update_permission_by_video_id(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD SERVICE]: - ❌️ 修改权限失败: {}", e))?;

        let video_info = VideoInfo::from_entity(video_entity);

        Ok(video_info)
    }

    ////////

    /// # 5. [SERVICE] - 删除一个视频
    /// * `desc`: `删除视频 + 更新计数`
    pub async fn del_one_video_and_update_count(video_id: i64) -> Result<bool, anyhow::Error> {
        match VideoAddRepository::pg_delete_video_by_id(video_id).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("RowNotFound") {
                    Ok(false)
                } else {
                    Err(anyhow::anyhow!(
                        "[🤐 ADD SERVICE]: - ❌️ 删除视频失败: {}",
                        e
                    ))
                }
            }
        }
    }

    ////////

    /// # 6. [SERVICE] - 批量删除视频
    /// * `desc`: `批量删除视频 + 更新计数`
    pub async fn del_many_video_and_update_count(
        video_ids: Vec<i64>,
    ) -> Result<bool, anyhow::Error> {
        match VideoAddRepository::pg_delete_video_by_ids(video_ids).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("RowNotFound") {
                    Ok(false)
                } else {
                    Err(anyhow::anyhow!(
                        "[🤐 ADD SERVICE]: - ❌️ 批量删除视频失败: {}",
                        e
                    ))
                }
            }
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
            .map_err(|e| anyhow::anyhow!("[🤐 ADD SERVICE]: - ❌️ 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD SERVICE]: - ❌️ 获取热门视频列表失败: {}", e))
    }

    ////////

    /// # 12. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD SERVICE]: - ❌️ 获取推荐视频列表失败: {}", e))
    }

    ////////
}

//////// END
