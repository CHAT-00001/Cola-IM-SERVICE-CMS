// repo_adapter/src/market/address.rs  -- 适配器 - 地址簿
// 2026/6/18

//////

use async_trait::async_trait;
use chrono::Utc;
use cola_data::market::command::address::AddressCommand;
use cola_data::market::info::address::AddressInfo;
use cola_data::market::port::address::AddressPort;
use repo::market::pg::address::AddressRepo;
use cola_data::market::entity::address::AddressEntity;

//////

/// # [ADAPTER] - 地址簿 端口适配器
pub struct AddressAdapter;

#[async_trait]
impl AddressPort for AddressAdapter {

    async fn save_address(&self, uid: i64, cmd: AddressCommand) -> anyhow::Result<()> {
        cmd.validate()?;
        let now = Utc::now().timestamp() as i32;
        let entity = AddressEntity {
            uid,
            name: cmd.name,
            country: cmd.country,
            province: cmd.province,
            city: cmd.city,
            area: cmd.area,
            address: cmd.address,
            area_code: String::new(),
            phone: cmd.phone,
            is_default: if cmd.is_default { 1 } else { 0 },
            add_time: now,
            upd_time: now,
            ..Default::default()
        };
        AddressRepo::insert(&entity).await?;
        Ok(())
    }

    async fn update_address(&self, uid: i64, address_id: i64, cmd: AddressCommand) -> anyhow::Result<()> {
        let now = Utc::now().timestamp() as i32;
        let entity = AddressEntity {
            uid,
            name: cmd.name,
            country: cmd.country,
            province: cmd.province,
            city: cmd.city,
            area: cmd.area,
            address: cmd.address,
            area_code: String::new(),
            phone: cmd.phone,
            is_default: if cmd.is_default { 1 } else { 0 },
            upd_time: now,
            ..Default::default()
        };
        AddressRepo::update(address_id, &entity).await?;
        Ok(())
    }

    async fn setting_default(&self, uid: i64, address_id: i64) -> anyhow::Result<()> {
        AddressRepo::set_default(uid, address_id).await?;
        Ok(())
    }

    async fn delete_address(&self, _uid: i64, address_id: i64) -> anyhow::Result<()> {
        AddressRepo::soft_delete(address_id).await?;
        Ok(())
    }

    async fn get_address_by_user_id(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<AddressInfo>> {
        let entities = AddressRepo::find_by_uid(uid, offset, limit).await?;
        Ok(entities.into_iter().map(AddressInfo::from).collect())
    }

    async fn get_address_by_id(&self, uid: i64, address_id: i64) -> anyhow::Result<AddressInfo> {
        let entity = AddressRepo::find_by_id(uid, address_id).await?
            .ok_or_else(|| anyhow::anyhow!("地址不存在"))?;
        Ok(AddressInfo::from(entity))
    }

    async fn delete_address_by_user_id(&self, _uid: i64, user_id: i64) -> anyhow::Result<()> {
        AddressRepo::delete_by_uid(user_id).await?;
        Ok(())
    }
}
