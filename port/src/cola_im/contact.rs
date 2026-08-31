// port/src/cola_im/contact.rs
// ⏩️ 端口 - 可乐IM - 联系人
// 2026-07-07 14:10 Created.

////////

use cola_data::cola_im::command::contact::ContactCommand;
use cola_data::cola_im::info::contact::ContactInfo;

////////

#[async_trait::async_trait]
pub trait ContactRepo: Send + Sync {
    /// 添加联系人
    async fn add_contact(&self, uid: i64, cmd: ContactCommand) -> anyhow::Result<()>;

    /// 同步联系人列表（增量拉取）
    async fn sync_contacts(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<ContactInfo>>;

    /// 删除联系人
    async fn del_contact(&self, uid: i64, card_id: i64) -> anyhow::Result<()>;

    /// 星标联系人
    async fn star_contact(&self, uid: i64, card_id: i64, is_stared: i16) -> anyhow::Result<()>;

    /// 特别关心
    async fn favorites_contact(
        &self,
        uid: i64,
        card_id: i64,
        favorites: bool,
    ) -> anyhow::Result<()>;

    /// 拉黑
    async fn block_contact(&self, uid: i64, card_id: i64, blocked: bool) -> anyhow::Result<()>;
}

//////// END
