// repo_adapter/src/user/add.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::user::command::new::UserCommand;
use cola_data::user::command::update_user::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::add::AddPort;

////////

pub struct UserAddPortAdapter;

#[async_trait]
impl AddPort for UserAddPortAdapter {
    /// # [ADAPTER] - 保存用户资料
    async fn save_user(&self, _cmd: UserCommand) -> anyhow::Result<UserInfo> {
        // 1. 在这里执行数据库插入 (INSERT)
        // 2. 获取数据库返回的新数据
        // 3. 将 PO 转换为 UserInfo 返回

        // 示例：这里先返回一个默认值，你需要替换为真实的数据库操作结果
        Ok(UserInfo::default())
    }

    /// # [ADAPTER] - 更新用户资料
    async fn update_user(&self, _cmd: UpdateUserCommand) -> anyhow::Result<UserInfo> {
        // 1. 在这里执行数据库更新 (UPDATE)
        // 2. 将更新后的 PO 转换为 UserInfo 返回

        Ok(UserInfo::default())
    }

    async fn del_one_user(&self, _user_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn del_many_user(&self, _user_ids: Vec<i64>) -> anyhow::Result<()> {
        Ok(())
    }
}
