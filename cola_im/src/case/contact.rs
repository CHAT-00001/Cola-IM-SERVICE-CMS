// cola_im/src/case/contact.rs  -- IM - case - 联系人
// 2026-07-07

//////

use anyhow::Result;
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_im::command::contact::ContactCommand;

//////

/// # [USE CASE] - 联系人 用例
pub struct ContactCase;

impl ContactCase {
    ////////

    /// # 1. [CASE] - 添加联系人
    pub async fn case_add(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        let cmd = ContactCommand {
            owner_id: uid,
            card_id: url.video_id,
            ..Default::default()
        };
        ctx.im.contact.add_contact(uid, cmd).await?;
        Ok("添加联系人成功".to_string())
    }

    ////////

    /// # 2. [CASE] - 同步联系人
    pub async fn case_sync(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        // 增量同步使用 upd_time 做版本号
        let _contacts = ctx.im.contact.sync_contacts(uid, url.offset, url.limit).await?;
        Ok("同步联系人成功".to_string())
    }

    ////////

    /// # 3. [CASE] - 删除联系人
    pub async fn case_delete(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.im.contact.del_contact(uid, url.video_id).await?;
        Ok("删除联系人成功".to_string())
    }

    ////////

    /// # 4. [CASE] - 星标
    pub async fn case_star(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.im.contact.star_contact(uid, url.video_id, 1).await?;
        Ok("星标联系人成功".to_string())
    }

    ////////

    /// # 5. [CASE] - 特别关心
    pub async fn case_favorites(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.im.contact.favorites_contact(uid, url.video_id, true).await?;
        Ok("特别关心成功".to_string())
    }

    ////////

    /// # 6. [CASE] - 拉黑
    pub async fn case_blocked(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<String> {
        ctx.im.contact.block_contact(uid, url.video_id, true).await?;
        Ok("拉黑联系人成功".to_string())
    }
}