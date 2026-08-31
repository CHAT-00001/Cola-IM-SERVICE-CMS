// repository/src/cola_im/service/contact.rs
// 服务 - 可乐IM - 联系人 - 模块
// 2026-07-07

////////

use cola_data::cola_im::info::contact::ContactInfo;
use repository::cola_im::pg::contact::ImContactRepo;
////////

/// # [CONTACT SERVICE] - 联系人
/// * `desc`: `联系人服务`
pub struct ImContactService;

// 构造实现
impl ImContactService {
    pub async fn add_contact(
        uid: i64,
        card_id: i64,
        remark_name: Option<String>,
    ) -> anyhow::Result<()> {
        let entity = ImContactRepo::save_contact(uid, card_id, remark_name).await?;
        let _ = ContactInfo::from_entity(entity);
        Ok(())
    }

    pub async fn sync_contacts(
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<ContactInfo>> {
        let entities = ImContactRepo::find_contacts_by_uid(uid, offset, limit).await?;
        Ok(entities.into_iter().map(ContactInfo::from_entity).collect())
    }

    pub async fn del_contact(uid: i64, card_id: i64) -> anyhow::Result<()> {
        ImContactRepo::soft_delete_contact(uid, card_id).await?;
        Ok(())
    }

    pub async fn star_contact(uid: i64, card_id: i64, is_stared: i16) -> anyhow::Result<()> {
        ImContactRepo::update_star(uid, card_id, is_stared).await?;
        Ok(())
    }

    pub async fn favorites_contact(uid: i64, card_id: i64, favorites: bool) -> anyhow::Result<()> {
        ImContactRepo::update_favorites(uid, card_id, favorites).await?;
        Ok(())
    }

    pub async fn block_contact(uid: i64, card_id: i64, blocked: bool) -> anyhow::Result<()> {
        ImContactRepo::update_blocked(uid, card_id, blocked).await?;
        Ok(())
    }
}

//////// END
