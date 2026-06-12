// repo_adapter/src/user/add.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::add::AddPort;
use cola_data::user::command::user::UserCommand;

pub struct UserAddPortAdapter;

#[async_trait]
impl AddPort for UserAddPortAdapter {
    async fn save_user(&self, _cmd: UserCommand) -> anyhow::Result<()> {
        Ok(())
    }
    async fn edit_user(&self, _cmd: UserCommand) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_one_user(&self, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_many_user(&self, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}