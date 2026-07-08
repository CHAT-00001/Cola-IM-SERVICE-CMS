// cola_data/src/im/port/card.rs  -- IM - port - 名片
// 2026-07-07

use crate::im::command::card::CardCommand;
use crate::im::info::card::CardInfo;

#[async_trait::async_trait]
pub trait CardRepo: Send + Sync {
    /// 新建名片
    async fn new_card(&self, uid: i64, cmd: CardCommand) -> anyhow::Result<()>;

    /// 查看名片
    async fn get_card(&self, card_id: i64) -> anyhow::Result<CardInfo>;

    /// 修改名片
    async fn edit_card(&self, uid: i64, card_id: i64, cmd: CardCommand) -> anyhow::Result<()>;

    /// 删除名片
    async fn del_card(&self, uid: i64, card_id: i64) -> anyhow::Result<()>;

    /// 同步名片列表
    async fn sync_cards(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<CardInfo>>;
}