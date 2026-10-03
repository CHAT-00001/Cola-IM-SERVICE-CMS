// cola_user/src/case/user/home.rs -- USER - CASE - 用户主页聚合
// 2026-10-03 10:13 Created.

////////

use crate::assembler::user::{USER_HOME_APP_ID, UserHomeAssembleInput, build_user_home_response};
use anyhow::{Result, anyhow};
use cola_data::cola_user::response::profile::{ProfileRelation, ProfileSingleResponse};
use port::app::ctx::AppContext;
use tracing::{info, warn};

////////

/// # [HOME CASE] - 用户主页聚合用例
/// * `desc`: 查询公开资料、对象关系和当前阶段 CDN，并交给 Assembler 生成主页响应
pub struct UserHomeCase;

impl UserHomeCase {
    ////////

    /// # 1. [CASE] - 获取公开用户主页
    /// * `current_uid`: 当前访问者ID
    /// * `target_uid`: 目标主页用户ID
    /// * `ctx`: 全局上下文
    pub async fn case_get_public_home(
        current_uid: i64,
        target_uid: i64,
        ctx: &AppContext,
    ) -> Result<ProfileSingleResponse> {
        if target_uid <= 0 {
            return Err(anyhow!("目标用户ID无效: {}", target_uid));
        }

        let user_info = ctx
            .user
            .profile
            .get
            .single_get_info(target_uid)
            .await
            .map_err(|error| anyhow!("查询公开用户资料失败: {}", error))?;

        let cdn_domain = Self::resolve_home_cdn_domain(ctx).await;
        let relation = Self::resolve_relation(current_uid, target_uid, ctx).await;

        let response = build_user_home_response(UserHomeAssembleInput {
            user_info,
            profile_info: None,
            social: Default::default(),
            content: Default::default(),
            count: Default::default(),
            relation,
            cdn_domain,
        });

        info!(
            "[🗣️ USER HOME CASE]: ✅️ 用户主页聚合成功, current_uid={}, target_uid={}",
            current_uid, target_uid
        );
        Ok(response)
    }

    ////////

    /// # 2. [CASE] - 获取当前访问者与主页对象关系
    /// * `desc`: 关注、拉黑和访问关系接入已有端口，其余对象关系等待统一关系端口
    async fn resolve_relation(
        current_uid: i64,
        target_uid: i64,
        ctx: &AppContext,
    ) -> ProfileRelation {
        if current_uid <= 0 {
            return ProfileRelation::default();
        }

        ProfileRelation {
            is_following: ctx
                .user
                .follow
                .check
                .is_followed(current_uid, target_uid)
                .await
                .unwrap_or(false),
            is_blocked: ctx
                .user
                .black
                .check
                .is_blacked(current_uid, target_uid)
                .await
                .unwrap_or(false),
            is_visited: ctx
                .user
                .view
                .check
                .is_visited(current_uid, target_uid)
                .await
                .unwrap_or(false),
            ..ProfileRelation::default()
        }
    }

    ////////

    /// # 3. [CASE] - 解析用户主页 CDN
    /// * `desc`: 当前阶段沿用视频业务的 short-video Bucket 语义
    async fn resolve_home_cdn_domain(ctx: &AppContext) -> String {
        const DEFAULT_CDN_DOMAIN: &str = "https://cdn.shortvideo.com";

        match ctx
            .fs
            .bucket
            .get
            .get_bucket_by_app_id(USER_HOME_APP_ID)
            .await
        {
            Ok(Some(bucket)) => bucket
                .cdn_domain
                .filter(|domain| !domain.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_CDN_DOMAIN.to_string()),
            Ok(None) => DEFAULT_CDN_DOMAIN.to_string(),
            Err(error) => {
                warn!(
                    "[🤐 USER HOME CASE] - ❌️ 查询主页 CDN 失败，使用默认域名: app_id={}, error={}",
                    USER_HOME_APP_ID, error
                );
                DEFAULT_CDN_DOMAIN.to_string()
            }
        }
    }
}

//////// END
