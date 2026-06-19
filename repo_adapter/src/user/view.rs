// repo_adapter/src/user/view.rs
// 2026-06-12

use async_trait::async_trait;
use cola_data::user::port::view::ViewPort;
use cola_data::user::command::new::UserCommand;

pub struct UserViewPortAdapter;

#[async_trait]
impl ViewPort for UserViewPortAdapter {
    async fn save_view(&self, _cmd: UserCommand) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_view(&self, _cmd: UserCommand) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_one_user(&self, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
    async fn del_many_user(&self, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}