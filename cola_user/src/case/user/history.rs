// cola_user/src/case/user/history.rs -- 用户资料及历史用例
// 2026/9/23 Created.

////////

use anyhow::{Result, ensure};
use cola_data::app::page::PageInfo;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::info::user_history::UserHistoryInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 用户资料及历史
pub struct UserHistoryCase;

impl UserHistoryCase {
    pub async fn update(uid: i64, cmd: UpdateUserCommand, ctx: &AppContext) -> Result<UserInfo> {
        ensure!(uid > 0, "用户会话无效");
        ensure!(
            cmd.avatar.is_some()
                || cmd.nickname.is_some()
                || cmd.signature.is_some()
                || cmd.bg_img.is_some()
                || cmd.avatar_thumb.is_some()
                || cmd.sns_url.is_some()
                || cmd.email.is_some()
                || cmd.phone.is_some()
                || cmd.birthday.is_some()
                || cmd.lat.is_some()
                || cmd.lng.is_some(),
            "没有可更新的资料字段"
        );
        if let Some(nickname) = cmd.nickname.as_deref() {
            ensure!(!nickname.trim().is_empty(), "昵称不能为空");
            ensure!(
                nickname.trim().chars().count() <= 64,
                "昵称不能超过64个字符"
            );
        }
        if let Some(avatar) = cmd.avatar.as_deref() {
            ensure!(avatar.trim().len() <= 2048, "头像路径不能超过2048字节");
        }
        let mut result = ctx.user.profile.history.update_profile(uid, cmd).await?;
        let cdn_domain = super::avatar_cdn::resolve_avatar_cdn_domain(ctx).await;
        result.avatar_url = super::avatar_cdn::resolve_avatar_url(&result.avatar_url, &cdn_domain);
        info!("[🗣️ CASE] - ✅️ 用户资料更新成功: uid={}", uid);
        Ok(result)
    }

    pub async fn list(
        uid: i64,
        is_avatar: bool,
        page: i64,
        qty: i64,
        ctx: &AppContext,
    ) -> Result<(Vec<UserHistoryInfo>, PageInfo)> {
        ensure!(uid > 0, "用户会话无效");
        let (mut list, page) = ctx
            .user
            .profile
            .history
            .list(uid, is_avatar, page.max(1), qty.clamp(1, 50))
            .await?;
        if is_avatar {
            let cdn_domain = super::avatar_cdn::resolve_avatar_cdn_domain(ctx).await;
            for item in &mut list {
                item.value = super::avatar_cdn::resolve_avatar_url(&item.value, &cdn_domain);
            }
        }
        Ok((list, page))
    }

    pub async fn activate(
        uid: i64,
        is_avatar: bool,
        id: i64,
        ctx: &AppContext,
    ) -> Result<UserInfo> {
        ensure!(uid > 0 && id > 0, "用户或历史记录 ID 无效");
        let mut result = ctx
            .user
            .profile
            .history
            .activate(uid, is_avatar, id)
            .await?;
        let cdn_domain = super::avatar_cdn::resolve_avatar_cdn_domain(ctx).await;
        result.avatar_url = super::avatar_cdn::resolve_avatar_url(&result.avatar_url, &cdn_domain);
        Ok(result)
    }

    pub async fn delete(uid: i64, is_avatar: bool, id: i64, ctx: &AppContext) -> Result<()> {
        ensure!(uid > 0 && id > 0, "用户或历史记录 ID 无效");
        ctx.user.profile.history.delete(uid, is_avatar, id).await
    }
}

//////// END
