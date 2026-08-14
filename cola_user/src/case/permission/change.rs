// user/src/case/permission/manage
// core - USER - case - permission -
// 2026/5/20 03:19

////////

use anyhow::Result;
use repository::cola_gis::service::permission_change::PermissionsChangeService;
use tracing::{info, warn};

////////

/// # [CASE] - 用户资料 修改 用例
pub struct ChangeCase;

impl ChangeCase {
    //

    ////////

    /// # 2. [CASE] - 修改昵称
    pub async fn case_change_nickname(
        uid: i64,
        user_id: i64,
        visibility_perm: i16, // 改名，与 Service 层保持一致
    ) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&visibility_perm) {
            anyhow::bail!("BIZ: 浏览权限值必须在1-5之间，当前值: {}", visibility_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, visibility_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 浏览权限同步失败: {}", e))?;

        info!(
            "BIZ - 浏览权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, visibility_perm
        );
        Ok(true)
    }

    ////////

    /// # 3. [CASE] - 修改个签
    pub async fn case_change_signature(uid: i64, user_id: i64, comment_perm: i16) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&comment_perm) {
            anyhow::bail!("BIZ: 评论权限值必须在1-5之间，当前值: {}", comment_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 评论权限同步失败: {}", e))?;

        info!(
            "BIZ - 评论权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, comment_perm
        );
        Ok(true)
    }

    ////////

    /// # 4. [CASE] - 修改头像
    pub async fn case_change_avatar(uid: i64, user_id: i64, danmaku_perm: i16) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&danmaku_perm) {
            anyhow::bail!("BIZ: 弹幕权限值必须在1-5之间，当前值: {}", danmaku_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, danmaku_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 弹幕权限同步失败: {}", e))?;

        info!(
            "BIZ - 弹幕权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, danmaku_perm
        );
        Ok(true)
    }

    //////

    /// # 5. [CASE] - 修改背景图
    pub async fn case_change_bg_img(uid: i64, user_id: i64, collect_perm: i16) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&collect_perm) {
            anyhow::bail!("BIZ: 收藏权限值必须在1-5之间，当前值: {}", collect_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, collect_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 收藏权限同步失败: {}", e))?;

        info!(
            "BIZ - 收藏权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, collect_perm
        );
        Ok(true)
    }

    ////////

    /// # 6. [CASE] - 修改生日
    pub async fn case_change_birthday(
        uid: i64,
        user_id: i64,
        download_perm: i16, // 改名，与 Service 层保持一致
    ) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&download_perm) {
            anyhow::bail!("BIZ: 下载权限值必须在1-5之间，当前值: {}", download_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, download_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 下载权限同步失败: {}", e))?;

        info!(
            "BIZ - 下载权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, download_perm
        );
        Ok(true)
    }

    ////////

    /// # 7. [CASE] - 购买权限
    pub async fn case_change_buy_perm(uid: i64, user_id: i64, buy_perm: i16) -> Result<bool> {
        // 验证权限值范围（可选，可以在用例层也做一次校验）
        if !(1..=5).contains(&buy_perm) {
            anyhow::bail!("BIZ: 购买权限值必须在1-5之间，当前值: {}", buy_perm);
        }

        // 修改视频评论权限
        PermissionsChangeService::update_video_comment_perm(uid, user_id, buy_perm)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 购买权限同步失败: {}", e))?;

        info!(
            "BIZ - 评论权限修改成功: uid={}, user_id={}, perm={}",
            uid, user_id, buy_perm
        );
        Ok(true)
    }

    ////////
}

//////// END
