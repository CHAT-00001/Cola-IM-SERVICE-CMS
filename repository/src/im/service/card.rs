// repository/src/im/service/card.rs  -- 仓储 - IM - Service - 名片
// 2026-07-07

use crate::im::pg::card::ImCardRepo;
use cola_data::im::info::card::CardInfo;

pub struct ImCardService;

impl ImCardService {
    pub async fn new_card(uid: i64, first_name: &str, last_name: &str, content: &str) -> anyhow::Result<()> {
        let entity = ImCardRepo::save_card(uid, first_name, last_name, content).await?;
        let _ = CardInfo::from_entity(entity);
        Ok(())
    }

    pub async fn get_card(card_id: i64) -> anyhow::Result<CardInfo> {
        let entity = ImCardRepo::find_card_by_id(card_id).await?
            .ok_or_else(|| anyhow::anyhow!("名片不存在: {}", card_id))?;
        Ok(CardInfo::from_entity(entity))
    }

    pub async fn sync_cards(uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<CardInfo>> {
        let entities = ImCardRepo::find_cards_by_uid(uid, offset, limit).await?;
        Ok(entities.into_iter().map(CardInfo::from_entity).collect())
    }

    pub async fn del_card(uid: i64, card_id: i64) -> anyhow::Result<()> {
        ImCardRepo::delete_card(uid, card_id).await?;
        Ok(())
    }
}