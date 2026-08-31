 // repo_adapter/src/cola_im/contact.rs -- 🔌 适配器 - 可乐IM - 联系人
// 2026-07-07 12:01

////////

use async_trait::async_trait;
use cola_data::cola_im::command::contact::ContactCommand;
use cola_data::cola_im::info::contact::ContactInfo;
use port::cola_im::contact::ContactRepo;
use repository::cola_im::service::contact::ImContactService;

////////

/// # [ADAPTER] - 联系人 - 适配器
/// * `desc`: `IM - 联系人适配器`
pub struct ContactPortAdapter;

#[async_trait]
impl ContactRepo for ContactPortAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 添加联系人
    async fn add_contact(&self, uid: i64, _cmd: ContactCommand) -> anyhow::Result<()> {
        ImContactService::add_contact(uid, _cmd.card_id, _cmd.remark_name).await
    }

    ////////

    /// # 2. [ADAPTER] - 同步联系人
    async fn sync_contacts(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<ContactInfo>> {
        ImContactService::sync_contacts(uid, offset, limit).await
    }

    ////////

    /// # 3. [ADAPTER] - 删除联系人
    async fn del_contact(&self, uid: i64, card_id: i64) -> anyhow::Result<()> {
        ImContactService::del_contact(uid, card_id).await
    }

    ////////

    /// # 4. [ADADPTER] - 星标联系人
    async fn star_contact(&self, uid: i64, card_id: i64, is_stared: i16) -> anyhow::Result<()> {
        ImContactService::star_contact(uid, card_id, is_stared).await
    }

    ////////

    /// # 5. [ADAPTER] - 最爱联系人
    async fn favorites_contact(
        &self,
        uid: i64,
        card_id: i64,
        favorites: bool,
    ) -> anyhow::Result<()> {
        ImContactService::favorites_contact(uid, card_id, favorites).await
    }

    ////////

    /// # 6. [ADAPTER] - 加入黑名单
    async fn block_contact(&self, uid: i64, card_id: i64, blocked: bool) -> anyhow::Result<()> {
        ImContactService::block_contact(uid, card_id, blocked).await
    }
}

//////// END
