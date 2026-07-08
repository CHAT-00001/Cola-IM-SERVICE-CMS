// repo_adapter/src/im/card.rs  -- 适配器 - IM - 名片
// 2026-07-07 18:00

////////

use async_trait::async_trait;
use cola_data::im::port::card::CardRepo;
use cola_data::im::command::card::CardCommand;
use cola_data::im::info::card::CardInfo;
use repo::im::service::card::ImCardService;

////////
pub struct CardPortAdapter;

#[async_trait]
impl CardRepo for CardPortAdapter {
    ////////

    /// # 1. 💾 新建
    async fn new_card(&self, uid: i64, _cmd: CardCommand) -> anyhow::Result<()> {
        ImCardService::new_card(uid, &_cmd.first_name, &_cmd.last_name, &_cmd.content).await
    }

    ////////

    /// # 2. ▶ 获取
    async fn get_card(&self, card_id: i64) -> anyhow::Result<CardInfo> {
        ImCardService::get_card(card_id).await
    }

    ////////

    /// # 3. 🍚 修改
    async fn edit_card(&self, _uid: i64, _card_id: i64, _cmd: CardCommand) -> anyhow::Result<()> {
        Ok(()) // TODO: implement
    }

    ////////

    /// # 4. ❌️ 用户软删除
    async fn del_card(&self, uid: i64, card_id: i64) -> anyhow::Result<()> {
        ImCardService::del_card(uid, card_id).await
    }

    ////////

    /// # 5. ❌️ SAVE 保存
    async fn sync_cards(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<CardInfo>> {
        ImCardService::sync_cards(uid, offset, limit).await
    }
}

////////
