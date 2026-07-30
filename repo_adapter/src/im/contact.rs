// repo_adapter/src/im/contact.rs  -- 适配器 - IM - 联系人
// 2026-07-07 12:01

////////

use async_trait::async_trait;
use cola_data::im::port::contact::ContactRepo;
use cola_data::im::command::contact::ContactCommand;
use cola_data::im::info::contact::ContactInfo;
use repository::im::service::contact::ImContactService;

////////


/// # [ADAPTER] - 联系人 - 适配器
pub struct ContactPortAdapter;

#[async_trait]
impl ContactRepo for ContactPortAdapter {
    async fn add_contact(&self, uid: i64, _cmd: ContactCommand) -> anyhow::Result<()> {
        ImContactService::add_contact(uid, _cmd.card_id, _cmd.remark_name).await
    }

    async fn sync_contacts(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<ContactInfo>> {
        ImContactService::sync_contacts(uid, offset, limit).await
    }

    async fn del_contact(&self, uid: i64, card_id: i64) -> anyhow::Result<()> {
        ImContactService::del_contact(uid, card_id).await
    }

    async fn star_contact(&self, uid: i64, card_id: i64, is_stared: i16) -> anyhow::Result<()> {
        ImContactService::star_contact(uid, card_id, is_stared).await
    }

    async fn favorites_contact(&self, uid: i64, card_id: i64, favorites: bool) -> anyhow::Result<()> {
        ImContactService::favorites_contact(uid, card_id, favorites).await
    }

    async fn block_contact(&self, uid: i64, card_id: i64, blocked: bool) -> anyhow::Result<()> {
        ImContactService::block_contact(uid, card_id, blocked).await
    }
}

//////// END