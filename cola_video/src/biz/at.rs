// cola_video/src/biz/at.rs  -- VIDEO - 逻辑层 - 我的
// 2026/06/05 01:00 by wx: cestbon10080

////////

use anyhow::{Context, Result};
use cola_data::app::request::ApiUrlParamsQuery;
use crate::assembler::video::build_video_list_response;
use crate::model::vo::video::VideoListResponse;
use repo::video::service::video::VideoService; // 👈 统一注入静态视频服务
use repo::video::service::user::UserService;   // 👈 统一注入静态用户/社交服务

////////

pub struct VideoAtLogic;

impl VideoAtLogic {

    /// # 1. [LOGIC] - 关注人动态流
    pub async fn logic_get_following_list(
        query: ApiUrlParamsQuery,
        current_uid: i64,
    ) -> Result<VideoListResponse> {
        // 1. 静态调用用户服务，获取该用户关注的所有博主 UID 集合
        let user_ids = UserService::find_following_ids(current_uid)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取关注列表失败: {}", e))?;

        if user_ids.is_empty() {
            return build_video_list_response(vec![], Some(current_uid), None, query.offset, query.limit, 0).await;
        }

        // 2. 静态调用视频服务，批量捞出这些博主的视频列表
        let entities = VideoService::find_video_by_user_ids(user_ids, query.offset, query.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取关注的人发表的视频列表失败: {}", e))?;

        // 3. 组装 VO 返回
        let models = build_video_list_response(
            entities,
            Some(current_uid),
            None,
            query.offset,
            query.limit,
            0,
        )
            .await?;

        Ok(models)
    }

    ////////

    /// # 2. [LOGIC] - 朋友流 (双向关注)
    pub async fn logic_get_friend_list(
        query: ApiUrlParamsQuery,
    ) -> Result<VideoListResponse> {
        let current_uid = query.uid.context("BIZ: 未登录无法查看朋友动态")?;

        // 1. 静态调用用户社交服务，捞出互相关注的好友 UID 集合
        let user_ids = UserService::find_friend_ids(current_uid)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取朋友列表失败: {}", e))?;

        if user_ids.is_empty() {
            return build_video_list_response(vec![], Some(current_uid), None, query.offset, query.limit, 0).await;
        }

        // 2. 批量捞视频
        let entities = VideoService::find_video_by_user_ids(user_ids, query.offset, query.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取朋友发表的视频列表失败: {}", e))?;

        let models = build_video_list_response(
            entities,
            Some(current_uid),
            None,
            query.offset,
            query.limit,
            0
        ).await?;

        Ok(models)
    }

    ////////

    /// # 3. [LOGIC] - 推荐 (个性推荐)
    pub async fn logic_get_recommend_list(
        query: ApiUrlParamsQuery,
    ) -> Result<VideoListResponse> {
        let current_uid = query.uid.context("BIZ: 未登录无法查看推荐")?;

        // 直接对接已经写好的 VideoService 静态推荐函数
        let entities = VideoService::find_recommend_video_list(query.limit, query.offset).await?;

        let models = build_video_list_response(
            entities,
            Some(current_uid),
            None,
            query.offset,
            query.limit,
            0
        ).await?;

        Ok(models)
    }

    ////////

    /// # 4. [LOGIC] - 我的附近
    pub async fn logic_get_nearby_list(
        query: ApiUrlParamsQuery,
    ) -> Result<VideoListResponse> {
        let current_uid = query.uid.context("BIZ: 未登录无法查看附近")?;
        let lat = query.lat.unwrap_or(0.0);
        let lng = query.lng.unwrap_or(0.0);

        // 对接 VideoService 静态同城/附近函数
        let rows = VideoService::find_city_video_list(lat, lng, query.limit, query.offset).await?;

        // 剥离 Row 承接的物理 Entity
        let entities = rows.into_iter().map(|r| r.entity).collect::<Vec<_>>();

        let models = build_video_list_response(
            entities,
            Some(current_uid),
            None,
            query.offset,
            query.limit,
            0
        ).await?;

        Ok(models)
    }
}

//////// END