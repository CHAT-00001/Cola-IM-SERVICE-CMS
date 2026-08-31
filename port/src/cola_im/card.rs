// port/src/cola_im/card.rs
// ⏩️ 端口 - 可乐IM - 名片
// 2026-07-07

////////

use cola_data::cola_im::command::card::CardCommand;
use cola_data::cola_im::info::card::CardInfo;

////////

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

//////// END
